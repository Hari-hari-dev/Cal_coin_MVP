use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, Mint, MintTo, Token, TokenAccount},
};
use std::str::FromStr;

// Replace with your actual Program ID
declare_id!("ADM5ikM5LS1ptrwFqXNyZDYazDzThSJknLNpJyw1x6c");

// Seeds
pub const TICKET_SEED: &[u8] = b"ticket";
pub const MINT_AUTH_SEED: &[u8] = b"mint_authority";

// -------------------------------------------------------------------
// PROGRAM
// -------------------------------------------------------------------
#[program]
pub mod daily_facescan {
    use super::*;

    /// (1) Initialize: Creates the Airdrop account & a new SPL Mint.
    /// Hard-coded daily_amount = 1440.
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let data = &mut ctx.accounts.airdrop;

        // 1) Check if already initialized
        if data.initialized {
            msg!("Airdrop is already initialized; aborting.");
            return err!(ErrorCode::AlreadyInitialized);
        }

        // 2) Hard-coded Gatekeeper Network (Civic). If you don’t need it, remove or customize.
        let fixed_gatekeeper_network = Pubkey::from_str("uniqobk8oGh4XBLMqM68K8M2zNu3CdYX7q5go7whQiv")
            .map_err(|_| error!(ErrorCode::InvalidPubkey))?;

        data.gatekeeper_network = fixed_gatekeeper_network;
        data.mint = ctx.accounts.mint.key();
        data.daily_amount = 1440;
        data.last_claim_timestamp = 0;

        // Initialize owners array. We'll consider the payer's key as the first owner
        data.owners[0] = ctx.accounts.authority.key();
        data.owners_count = 1;
        for i in 1..data.owners.len() {
            data.owners[i] = Pubkey::default();
        }

        // Mark as initialized
        data.initialized = true;
        Ok(())
    }

    /// (2) Claim: rely only on the `signer` (the same as `payer`) for gating checks.
    /// Removes the need for a `gatewayToken` argument entirely.
    pub fn claim(ctx: Context<Claim>) -> Result<()> {
        let data = &mut ctx.accounts.airdrop;

        // 1) A naive gating check: require that the signer's key
        // matches some condition. For example, check if user is in
        // owners array, or something else. Or skip if you only want
        // the signer's signature to be enough. Up to you:
        /*
        if !is_authorized(&ctx.accounts.payer.key(), data) {
            msg!("Signer is not authorized for claim!");
            return err!(ErrorCode::Unauthorized);
        }
        */

        // 2) Time-based logic
        let now = Clock::get()?.unix_timestamp;
        let mut delta = now - data.last_claim_timestamp;
        if delta < 0 {
            delta = 0;
        }
        // cap at 7 days
        if delta > 7 * 86400 {
            delta = 7 * 86400;
        }
        let tokens_per_second = data.daily_amount as f64 / 86400.0;
        let minted_float = tokens_per_second * (delta as f64);
        let minted_amount = minted_float.floor() as u64;

        data.last_claim_timestamp = now;

        // 3) Mint if minted_amount > 0
        if minted_amount > 0 {
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

            msg!("Claimed {} tokens!", minted_amount);
        } else {
            msg!("No tokens minted (insufficient time).");
        }

        Ok(())
    }

    /// (3) Add a new owner if space is available
    pub fn add_owner(ctx: Context<AddOwner>, new_owner: Pubkey) -> Result<()> {
        add_owner_logic(ctx, new_owner)
    }

    /// (4) Delete an existing owner
    pub fn delete_owner(ctx: Context<DeleteOwner>, target_owner: Pubkey) -> Result<()> {
        delete_owner_logic(ctx, target_owner)
    }

    /// (5) Change the gatekeeper network if the signer is an owner
    pub fn change_gateway_network(ctx: Context<ChangeGateway>, new_gatekeeper: Pubkey) -> Result<()> {
        change_gateway_logic(ctx, new_gatekeeper)
    }
}

// -------------------------------------------------------------------
// ACCOUNTS
// -------------------------------------------------------------------
#[derive(Accounts)]
pub struct Initialize<'info> {
    // The main state account
    #[account(
        init,
        payer = authority,
        space = Airdrop::SIZE
    )]
    pub airdrop: Account<'info, Airdrop>,

    // Create a new SPL Mint with decimals=9, authority = mint_authority (the PDA)
    #[account(
        init,
        payer = authority,
        mint::decimals = 9,
        mint::authority = mint_authority,
    )]
    pub mint: Account<'info, Mint>,

    // Derive via seeds = [airdrop, "mint_authority"]
    #[account(
        seeds = [airdrop.key().as_ref(), MINT_AUTH_SEED],
        bump
    )]
    pub mint_authority: SystemAccount<'info>,

    // The wallet paying for creation
    #[account(mut)]
    pub authority: Signer<'info>,

    // Programs
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct Claim<'info> {
    // Must have same mint
    #[account(has_one = mint)]
    pub airdrop: Account<'info, Airdrop>,

    #[account(mut)]
    pub payer: Signer<'info>, // The main signer/gater

    #[account(
        seeds = [airdrop.key().as_ref(), MINT_AUTH_SEED],
        bump
    )]
    pub mint_authority: SystemAccount<'info>,

    // Ticket for uniqueness or optional tracking
    #[account(
        init,
        payer = payer,
        seeds = [airdrop.key().as_ref(), payer.key().as_ref(), TICKET_SEED],
        bump,
        space = Ticket::SIZE
    )]
    pub ticket: Account<'info, Ticket>,

    #[account(mut)]
    pub mint: Account<'info, Mint>,

    // The user’s associated token account
    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = mint,
        associated_token::authority = payer
    )]
    pub recipient_token_account: Account<'info, TokenAccount>,

    // Programs
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

// Additional instructions if needed
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
    pub gatekeeper_network: Pubkey, // If you want a network reference
    pub mint: Pubkey,               // The minted SPL
    pub daily_amount: u64,
    pub last_claim_timestamp: i64,
    // owners array if you want multi-sig ownership
    pub owners: [Pubkey; 6],
    pub owners_count: u8,
    // Freeze re-init
    pub initialized: bool
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
    #[msg("Invalid gating or pass check not satisfied")]
    InvalidPass,

    #[msg("Not an authorized owner")]
    Unauthorized,

    #[msg("Owners array is full")]
    OwnersFull,

    #[msg("Pubkey is already an owner")]
    AlreadyOwner,

    #[msg("Owner not found in the array")]
    OwnerNotFound,

    #[msg("Cannot remove yourself")]
    CannotRemoveSelf,

    #[msg("Could not parse gatekeeper network as a valid Pubkey")]
    InvalidPubkey,

    #[msg("Already initialized")]
    AlreadyInitialized,
}

// -------------------------------------------------------------------
// HELPER
// -------------------------------------------------------------------
fn is_authorized(signer_pubkey: &Pubkey, airdrop: &Airdrop) -> bool {
    for i in 0..airdrop.owners_count {
        if airdrop.owners[i as usize] == *signer_pubkey {
            return true;
        }
    }
    false
}

// Ownership logic
fn add_owner_logic(ctx: Context<AddOwner>, new_owner: Pubkey) -> Result<()> {
    let ad = &mut ctx.accounts.airdrop;
    let signer_key = ctx.accounts.signer.key();

    require!(is_authorized(&signer_key, ad), ErrorCode::Unauthorized);
    require!(ad.owners_count < 6, ErrorCode::OwnersFull);
    if new_owner == signer_key {
        return err!(ErrorCode::AlreadyOwner);
    }
    for i in 0..ad.owners_count {
        if ad.owners[i as usize] == new_owner {
            return err!(ErrorCode::AlreadyOwner);
        }
    }
    ad.owners[ad.owners_count as usize] = new_owner;
    ad.owners_count += 1;
    msg!("Added new owner: {}", new_owner);
    Ok(())
}

fn delete_owner_logic(ctx: Context<DeleteOwner>, target_owner: Pubkey) -> Result<()> {
    let ad = &mut ctx.accounts.airdrop;
    let signer_key = ctx.accounts.signer.key();

    require!(is_authorized(&signer_key, ad), ErrorCode::Unauthorized);

    if target_owner == signer_key {
        return err!(ErrorCode::CannotRemoveSelf);
    }
    let mut found_index = None;
    for i in 0..ad.owners_count {
        if ad.owners[i as usize] == target_owner {
            found_index = Some(i as usize);
            break;
        }
    }
    let idx = match found_index {
        Some(i) => i,
        None => return err!(ErrorCode::OwnerNotFound),
    };
    let last_idx = ad.owners_count as usize - 1;
    if idx != last_idx {
        ad.owners[idx] = ad.owners[last_idx];
    }
    ad.owners[last_idx] = Pubkey::default();
    ad.owners_count -= 1;

    msg!("Deleted owner: {}", target_owner);
    Ok(())
}

fn change_gateway_logic(ctx: Context<ChangeGateway>, new_gatekeeper: Pubkey) -> Result<()> {
    let ad = &mut ctx.accounts.airdrop;
    require!(is_authorized(&ctx.accounts.signer.key(), ad), ErrorCode::Unauthorized);

    ad.gatekeeper_network = new_gatekeeper;
    msg!("Updated gatekeeper network => {}", new_gatekeeper);
    Ok(())
}
