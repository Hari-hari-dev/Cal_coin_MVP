use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{
        self,
        Mint,
        MintTo,
        Token,
        TokenAccount,
    },
};
use solana_gateway::Gateway;
use std::str::FromStr; // For Pubkey::from_str

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
    /// Hard-coded daily_amount = 1440. Also sets `initialized = true` to freeze re-init.
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let data = &mut ctx.accounts.airdrop;

        // 1) Check if already initialized
        if data.initialized {
            msg!("Airdrop is already initialized; aborting.");
            return err!(ErrorCode::AlreadyInitialized);
        }

        // 2) Hard-coded Gatekeeper Network
        let fixed_gatekeeper_network = Pubkey::from_str("uniqobk8oGh4XBLMqM68K8M2zNu3CdYX7q5go7whQiv")
            .map_err(|_| error!(ErrorCode::InvalidPubkey))?;

        data.gatekeeper_network = fixed_gatekeeper_network;
        data.mint = ctx.accounts.mint.key();
        data.daily_amount = 1440;
        data.last_claim_timestamp = 0;

        // Initialize owners array. We'll consider the payer's key as the first owner
        data.owners[0] = ctx.accounts.authority.key();
        data.owners_count = 1;
        // Zero out the rest
        for i in 1..data.owners.len() {
            data.owners[i] = Pubkey::default();
        }

        // Mark as initialized
        data.initialized = true;

        Ok(())
    }

    /// (2) Claim: Requires a valid gateway token + time-based daily logic + mint.
    pub fn claim(ctx: Context<Claim>) -> Result<()> {
        let data = &mut ctx.accounts.airdrop;

        // 1) Gateway token check (Civic gating).
        Gateway::verify_gateway_token_account_info(
            &ctx.accounts.gateway_token.to_account_info(),
            &ctx.accounts.recipient.key(),
            &data.gatekeeper_network,
            None
        ).map_err(|_e| {
            msg!("Gateway token verification failed");
            ProgramError::InvalidArgument
        })?;

        // 2) Time-based daily logic
        let now = Clock::get()?.unix_timestamp;
        let mut delta = now - data.last_claim_timestamp;
        if delta < 0 {
            delta = 0;
        }
        // Cap at 7 days
        if delta > 7 * 86400 {
            delta = 7 * 86400;
        }
        let tokens_per_second = data.daily_amount as f64 / 86400.0; // e.g. 1440 / 86400
        let minted_float = tokens_per_second * (delta as f64);
        let minted_amount = minted_float.floor() as u64;

        data.last_claim_timestamp = now;

        // 3) Mint if minted_amount > 0
        if minted_amount > 0 {
            // Derive the PDA seeds
            let airdrop_key = data.key();
            let seeds = &[
                airdrop_key.as_ref(),
                MINT_AUTH_SEED,
                &[ctx.bumps.mint_authority],
            ];
            let signer = &[&seeds[..]];

            // Mint to the user's token account
            token::mint_to(
                CpiContext::new_with_signer(
                    ctx.accounts.token_program.to_account_info(),
                    MintTo {
                        authority: ctx.accounts.mint_authority.to_account_info(),
                        to: ctx.accounts.recipient_token_account.to_account_info(),
                        mint: ctx.accounts.mint.to_account_info(),
                    },
                    signer,
                ),
                minted_amount,
            )?;

            msg!("Claimed {} tokens (gateway-gated)!", minted_amount);
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

    // PDA used to sign future mint instructions
    #[account(
        seeds = [airdrop.key().as_ref(), MINT_AUTH_SEED],
        bump
    )]
    pub mint_authority: SystemAccount<'info>,

    // The wallet paying for the Airdrop + Mint creation
    #[account(mut)]
    pub authority: Signer<'info>,

    // Programs
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct Claim<'info> {
    #[account(has_one = mint)]
    pub airdrop: Account<'info, Airdrop>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        seeds = [airdrop.key().as_ref(), MINT_AUTH_SEED],
        bump
    )]
    pub mint_authority: SystemAccount<'info>,

    #[account(
        init,
        payer = payer,
        seeds = [airdrop.key().as_ref(), recipient.key().as_ref(), TICKET_SEED],
        bump,
        space = Ticket::SIZE
    )]
    pub ticket: Account<'info, Ticket>,

    #[account(mut)]
    pub mint: Account<'info, Mint>,

    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = mint,
        associated_token::authority = recipient
    )]
    pub recipient_token_account: Account<'info, TokenAccount>,

    /// CHECK:
    /// We'll do a runtime check with solana_gateway
    #[account(mut)]
    pub gateway_token: UncheckedAccount<'info>,

    #[account(mut)]
    pub recipient: SystemAccount<'info>,

    // Programs
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,

    // Optionally verify system program if needed
    #[account(address = anchor_lang::system_program::ID)]
    pub system_prog: Program<'info, System>,
}

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
    pub gatekeeper_network: Pubkey, // For solana-gateway
    pub mint: Pubkey,               // The newly created Mint
    pub daily_amount: u64,          // Hard-coded to 1440 in `initialize`
    pub last_claim_timestamp: i64,

    // *All* owners live in a single array with same privileges
    pub owners: [Pubkey; 6],
    pub owners_count: u8,

    // Freeze re-init
    pub initialized: bool
}
impl Airdrop {
    // old ~281 + 1 (bool) => 282
    // Here we gave a bit more overhead (300) in case we want future expansions
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

    // Already init
    #[msg("Airdrop is already initialized")]
    AlreadyInitialized,
}

// -------------------------------------------------------------------
// HELPER FUNCTIONS
// -------------------------------------------------------------------
fn is_authorized(signer_pubkey: &Pubkey, airdrop: &Airdrop) -> bool {
    for i in 0..airdrop.owners_count {
        if airdrop.owners[i as usize] == *signer_pubkey {
            return true;
        }
    }
    false
}

// Logic for add_owner
fn add_owner_logic(ctx: Context<AddOwner>, new_owner: Pubkey) -> Result<()> {
    let airdrop = &mut ctx.accounts.airdrop;
    let signer_key = ctx.accounts.signer.key();

    // Must be an existing owner
    require!(is_authorized(&signer_key, airdrop), ErrorCode::Unauthorized);

    // Ensure we have space
    require!(airdrop.owners_count < 6, ErrorCode::OwnersFull);

    // Check for duplicates
    if new_owner == signer_key {
        return err!(ErrorCode::AlreadyOwner);
    }
    for i in 0..airdrop.owners_count {
        if airdrop.owners[i as usize] == new_owner {
            return err!(ErrorCode::AlreadyOwner);
        }
    }

    // Insert into the next free slot
    let idx = airdrop.owners_count as usize;
    airdrop.owners[idx] = new_owner;
    airdrop.owners_count += 1;

    msg!("Added new owner: {}", new_owner);
    Ok(())
}

// Logic for delete_owner
fn delete_owner_logic(ctx: Context<DeleteOwner>, target_owner: Pubkey) -> Result<()> {
    let airdrop = &mut ctx.accounts.airdrop;
    let signer_key = ctx.accounts.signer.key();

    // Must be an existing owner
    require!(is_authorized(&signer_key, airdrop), ErrorCode::Unauthorized);

    // Disallow removing yourself (optional)
    if target_owner == signer_key {
        return err!(ErrorCode::CannotRemoveSelf);
    }

    // Find target in owners
    let mut found_index = None;
    for i in 0..airdrop.owners_count {
        if airdrop.owners[i as usize] == target_owner {
            found_index = Some(i as usize);
            break;
        }
    }
    let idx = match found_index {
        Some(i) => i,
        None => return err!(ErrorCode::OwnerNotFound),
    };

    // Remove by swapping with last
    let last_idx = airdrop.owners_count as usize - 1;
    if idx != last_idx {
        airdrop.owners[idx] = airdrop.owners[last_idx];
    }
    airdrop.owners[last_idx] = Pubkey::default();
    airdrop.owners_count -= 1;

    msg!("Deleted owner: {}", target_owner);
    Ok(())
}

// Logic for change_gateway_network
fn change_gateway_logic(ctx: Context<ChangeGateway>, new_gatekeeper: Pubkey) -> Result<()> {
    let airdrop = &mut ctx.accounts.airdrop;
    let signer_key = ctx.accounts.signer.key();

    // Must be an existing owner
    require!(is_authorized(&signer_key, airdrop), ErrorCode::Unauthorized);

    airdrop.gatekeeper_network = new_gatekeeper;
    msg!("Changed gatekeeper network to {}", new_gatekeeper);

    Ok(())
}
