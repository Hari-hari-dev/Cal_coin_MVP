use anchor_lang::{
    prelude::*,
    system_program, // If needed for address checks or Program references
};
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{
        self, // For `mint_to`
        Mint,
        MintTo,
        Token,
        TokenAccount,
    },
};
use solana_gateway::Gateway; // For `verify_gateway_token_account_info`
use std::str::FromStr;

// The Program ID for your Anchor program
declare_id!("3ArwtqNnwiUys3GmGub1NUrb4sjVbRhKQq2pKVLiFhtB");

// Seeds for PDAs
pub const TICKET_SEED: &[u8] = b"ticket";
pub const MINT_AUTH_SEED: &[u8] = b"mint_authority";

// -------------------------------------------------------------------
// PROGRAM
// -------------------------------------------------------------------
#[program]
pub mod daily_facescan {
    use super::*;

    /// (1) Initialize: Creates the Airdrop account, a new SPL Mint, 
    /// and sets up gating (Civic network) + owners array if needed.
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let data = &mut ctx.accounts.airdrop;

        // Ensure not re-initialized
        if data.initialized {
            return err!(ErrorCode::AlreadyInitialized);
        }

        // Optionally set a gatekeeper network (Civic). Hard-coded example:
        let gatekeeper_network = Pubkey::from_str("uniqobk8oGh4XBLMqM68K8M2zNu3CdYX7q5go7whQiv")
            .map_err(|_| error!(ErrorCode::InvalidPubkey))?;
        data.gatekeeper_network = gatekeeper_network;

        // Hard-coded daily_amount, etc.
        data.mint = ctx.accounts.mint.key();
        data.daily_amount = 1440;
        data.last_claim_timestamp = 0;

        // The user/authority is stored as the first owner (optional multi-owner)
        data.owners[0] = ctx.accounts.authority.key();
        data.owners_count = 1;
        for i in 1..data.owners.len() {
            data.owners[i] = Pubkey::default();
        }
        data.initialized = true;

        msg!("Airdrop initialized successfully.");
        Ok(())
    }

    /// (2) Claim: checks Civic pass + time-based daily logic, then mints to user’s ATA.
    pub fn claim(ctx: Context<Claim>) -> Result<()> {
        let data = &mut ctx.accounts.airdrop;

        // 1) Check the user’s gateway token if you are gating with Civic
        Gateway::verify_gateway_token_account_info(
            &ctx.accounts.gateway_token.to_account_info(),
            &ctx.accounts.payer.key(),
            &data.gatekeeper_network,
            None
        )
        .map_err(|_err| {
            msg!("Civic gateway token verification failed.");
            error!(ErrorCode::InvalidPass)
        })?;

        // 2) Simple time-based daily logic
        let now = Clock::get()?.unix_timestamp;
        let mut delta = now - data.last_claim_timestamp;
        if delta < 0 {
            delta = 0;
        }
        // Cap at 7 days
        if delta > 7 * 86400 {
            delta = 7 * 86400;
        }
        let minted_float = (data.daily_amount as f64 / 86400.0) * (delta as f64);
        let minted_amount = minted_float.floor() as u64;

        data.last_claim_timestamp = now;

        // 3) Mint if minted_amount > 0
        if minted_amount > 0 {
            // Derive the PDA seeds for mint authority
            let airdrop_key = data.key();
            let seeds = &[
                airdrop_key.as_ref(),
                MINT_AUTH_SEED,
                &[ctx.bumps.mint_authority],
            ];
            let signer_seeds = &[&seeds[..]];

            token::mint_to(
                CpiContext::new_with_signer(
                    ctx.accounts.token_program.to_account_info(),
                    MintTo {
                        authority: ctx.accounts.mint_authority.to_account_info(),
                        to: ctx.accounts.recipient_token_account.to_account_info(),
                        mint: ctx.accounts.mint.to_account_info(),
                    },
                    signer_seeds,
                ),
                minted_amount,
            )?;

            msg!("Claimed {} tokens to ATA: {}", minted_amount, ctx.accounts.recipient_token_account.key());
        } else {
            msg!("No tokens minted (insufficient time).");
        }

        Ok(())
    }

    /// (3) Optional: Add a new owner if space is available.
    pub fn add_owner(ctx: Context<AddOwner>, new_owner: Pubkey) -> Result<()> {
        add_owner_logic(ctx, new_owner)
    }

    /// (4) Optional: Delete an existing owner
    pub fn delete_owner(ctx: Context<DeleteOwner>, target_owner: Pubkey) -> Result<()> {
        delete_owner_logic(ctx, target_owner)
    }

    /// (5) Optional: Change the gatekeeper network if signer is an owner
    pub fn change_gateway_network(ctx: Context<ChangeGateway>, new_gatekeeper: Pubkey) -> Result<()> {
        change_gateway_logic(ctx, new_gatekeeper)
    }
}

// -------------------------------------------------------------------
// ACCOUNTS
// -------------------------------------------------------------------
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = authority,
        space = Airdrop::SIZE
    )]
    pub airdrop: Account<'info, Airdrop>,

    #[account(
        init,
        payer = authority,
        mint::decimals = 9,
        mint::authority = mint_authority
    )]
    pub mint: Account<'info, Mint>,

    #[account(
        seeds = [airdrop.key().as_ref(), MINT_AUTH_SEED],
        bump
    )]
    pub mint_authority: SystemAccount<'info>, // PDA controlling the mint

    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

/// Claim: the user calls this to mint tokens. We also pass in a “gateway_token” for Civic gating.
#[derive(Accounts)]
pub struct Claim<'info> {
    // Must have same mint
    #[account(has_one = mint)]
    pub airdrop: Account<'info, Airdrop>,

    #[account(mut)]
    pub payer: Signer<'info>,

    // The PDA for mint authority
    #[account(
        seeds = [airdrop.key().as_ref(), MINT_AUTH_SEED],
        bump
    )]
    pub mint_authority: SystemAccount<'info>,

    // If you want to track a "Ticket" for uniqueness
    #[account(
        init,
        payer = payer,
        seeds = [airdrop.key().as_ref(), payer.key().as_ref(), crate::TICKET_SEED],
        bump,
        space = Ticket::SIZE
    )]
    pub ticket: Account<'info, Ticket>,

    #[account(mut)]
    pub mint: Account<'info, Mint>,

    // The user’s associated token account to receive minted tokens
    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = mint,
        associated_token::authority = payer
    )]
    pub recipient_token_account: Account<'info, TokenAccount>,

    // The Civic pass itself, e.g. gateway token account
    #[account(mut)]
    pub gateway_token: UncheckedAccount<'info>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

// Optional instructions for multi-owner
#[derive(Accounts)]
pub struct AddOwner<'info> {
    #[account(mut)]
    pub airdrop: Account<'info, Airdrop>,
    #[account(mut)]
    pub signer: Signer<'info>,
}
#[derive(Accounts)]
pub struct DeleteOwner<'info> {
    #[account(mut)]
    pub airdrop: Account<'info, Airdrop>,
    #[account(mut)]
    pub signer: Signer<'info>,
}
#[derive(Accounts)]
pub struct ChangeGateway<'info> {
    #[account(mut)]
    pub airdrop: Account<'info, Airdrop>,
    #[account(mut)]
    pub signer: Signer<'info>,
}

// -------------------------------------------------------------------
// DATA
// -------------------------------------------------------------------
#[account]
#[derive(Default)]
pub struct Airdrop {
    pub gatekeeper_network: Pubkey, // e.g. "uniqobk8..." if gating with Civic
    pub mint: Pubkey,               // The SPL mint created in `initialize`
    pub daily_amount: u64,          // e.g. 1440
    pub last_claim_timestamp: i64,

    pub owners: [Pubkey; 6],
    pub owners_count: u8,
    pub initialized: bool,
}
impl Airdrop {
    pub const SIZE: usize = 300;
}

#[account]
pub struct Ticket {}
impl Ticket {
    pub const SIZE: usize = 8;
}

// -------------------------------------------------------------------
// ERRORS
// -------------------------------------------------------------------
#[error_code]
pub enum ErrorCode {
    #[msg("Invalid gateway token or gating check not satisfied")]
    InvalidPass,

    #[msg("You are not an authorized owner")]
    Unauthorized,

    #[msg("Owners array is full")]
    OwnersFull,

    #[msg("That pubkey is already an owner")]
    AlreadyOwner,

    #[msg("Owner not found in the array")]
    OwnerNotFound,

    #[msg("Cannot remove yourself")]
    CannotRemoveSelf,

    #[msg("Could not parse gatekeeper network as a valid Pubkey")]
    InvalidPubkey,

    #[msg("Airdrop is already initialized")]
    AlreadyInitialized,
}

// If you want owners gating for claim, add a check for `is_authorized`.
fn is_authorized(signer_pubkey: &Pubkey, airdrop: &Airdrop) -> bool {
    for i in 0..airdrop.owners_count {
        if airdrop.owners[i as usize] == *signer_pubkey {
            return true;
        }
    }
    false
}

fn add_owner_logic(ctx: Context<AddOwner>, new_owner: Pubkey) -> Result<()> {
    let a = &mut ctx.accounts.airdrop;
    require!(is_authorized(&ctx.accounts.signer.key(), a), ErrorCode::Unauthorized);
    require!(a.owners_count < 6, ErrorCode::OwnersFull);

    // Disallow adding yourself or duplicates
    if new_owner == ctx.accounts.signer.key() {
        return err!(ErrorCode::AlreadyOwner);
    }
    for i in 0..a.owners_count {
        if a.owners[i as usize] == new_owner {
            return err!(ErrorCode::AlreadyOwner);
        }
    }
    let idx = a.owners_count as usize;
    a.owners[idx] = new_owner;
    a.owners_count += 1;
    msg!("Added new owner => {}", new_owner);
    Ok(())
}

fn delete_owner_logic(ctx: Context<DeleteOwner>, target_owner: Pubkey) -> Result<()> {
    let a = &mut ctx.accounts.airdrop;
    require!(is_authorized(&ctx.accounts.signer.key(), a), ErrorCode::Unauthorized);

    if target_owner == ctx.accounts.signer.key() {
        return err!(ErrorCode::CannotRemoveSelf);
    }
    let mut found_index = None;
    for i in 0..a.owners_count {
        if a.owners[i as usize] == target_owner {
            found_index = Some(i as usize);
            break;
        }
    }
    let idx = match found_index {
        Some(i) => i,
        None => return err!(ErrorCode::OwnerNotFound),
    };
    let last_idx = a.owners_count as usize - 1;
    if idx != last_idx {
        a.owners[idx] = a.owners[last_idx];
    }
    a.owners[last_idx] = Pubkey::default();
    a.owners_count -= 1;
    msg!("Deleted owner => {}", target_owner);
    Ok(())
}

fn change_gateway_logic(ctx: Context<ChangeGateway>, new_gatekeeper: Pubkey) -> Result<()> {
    let a = &mut ctx.accounts.airdrop;
    require!(is_authorized(&ctx.accounts.signer.key(), a), ErrorCode::Unauthorized);
    a.gatekeeper_network = new_gatekeeper;
    msg!("Changed gatekeeper network => {}", new_gatekeeper);
    Ok(())
}
