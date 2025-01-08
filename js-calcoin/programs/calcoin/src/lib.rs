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
use solana_gateway_anchor::Pass;  // <-- Key addition: bring in the Pass type
use std::str::FromStr; // For Pubkey::from_str if needed

// Replace with your actual Program ID (must match what's in Cargo.toml + declare_id!).
declare_id!("3ArwtqNnwiUys3GmGub1NUrb4sjVbRhKQq2pKVLiFhtB");

// Seeds for the PDAs
pub const TICKET_SEED: &[u8] = b"ticket";
pub const MINT_AUTH_SEED: &[u8] = b"mint_authority";

// -------------------------------------------------------------------
// PROGRAM
// -------------------------------------------------------------------
#[program]
pub mod daily_facescan {
    use super::*;

    /// (1) Initialize the program: creates the Airdrop account and a new SPL Mint.
    /// Hard-coded daily_amount = 1440. Also sets `initialized = true` to prevent re-initialization.
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let data = &mut ctx.accounts.airdrop;

        if data.initialized {
            msg!("Airdrop is already initialized; aborting.");
            return err!(ErrorCode::AlreadyInitialized);
        }

        // Hard-coded Gatekeeper Network for Civic pass gating
        let fixed_gatekeeper_network = Pubkey::from_str("uniqobk8oGh4XBLMqM68K8M2zNu3CdYX7q5go7whQiv")
            .map_err(|_| error!(ErrorCode::InvalidPubkey))?;
        data.gatekeeper_network = fixed_gatekeeper_network;

        data.mint = ctx.accounts.mint.key();
        data.daily_amount = 1440;
        data.last_claim_timestamp = 0;

        // Consider the payer's key as the first "owner"
        let signer_key = ctx.accounts.authority.key();
        data.owners[0] = signer_key;
        data.owners_count = 1;
        for i in 1..data.owners.len() {
            data.owners[i] = Pubkey::default();
        }

        data.initialized = true;
        Ok(())
    }

    /// (2) Claim tokens. We now rely on the `Pass` constraint to verify that
    /// the user holds a valid pass for `gatekeeper_network`. If no pass or invalid,
    /// the transaction fails.
    pub fn claim(ctx: Context<Claim>) -> Result<()> {
        let data = &mut ctx.accounts.airdrop;

        // Time-based daily logic
        let now = Clock::get()?.unix_timestamp;
        let mut delta = now - data.last_claim_timestamp;
        if delta < 0 {
            delta = 0;
        }
        // Cap at 7 days
        if delta > 7 * 86400 {
            delta = 7 * 86400;
        }
        let tokens_per_second = data.daily_amount as f64 / 86400.0;
        let minted_float = tokens_per_second * (delta as f64);
        let minted_amount = minted_float.floor() as u64;

        data.last_claim_timestamp = now;

        // Mint if minted_amount > 0
        if minted_amount > 0 {
            let airdrop_key = data.key();
            let seeds = &[
                airdrop_key.as_ref(),
                MINT_AUTH_SEED,
                &[ctx.bumps.mint_authority],
            ];
            let signer = &[&seeds[..]];

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

    /// (3) Add new owner if there's space
    pub fn add_owner(ctx: Context<AddOwner>, new_owner: Pubkey) -> Result<()> {
        add_owner_logic(ctx, new_owner)
    }

    /// (4) Delete existing owner
    pub fn delete_owner(ctx: Context<DeleteOwner>, target_owner: Pubkey) -> Result<()> {
        delete_owner_logic(ctx, target_owner)
    }

    /// (5) Change gateway network if the signer is an existing owner
    pub fn change_gateway_network(ctx: Context<ChangeGateway>, new_gatekeeper: Pubkey) -> Result<()> {
        change_gateway_logic(ctx, new_gatekeeper)
    }
}

// -------------------------------------------------------------------
// ACCOUNTS
// -------------------------------------------------------------------
#[derive(Accounts)]
pub struct Initialize<'info> {
    /// The main Airdrop state account
    #[account(
        init,
        payer = authority,
        space = Airdrop::SIZE
    )]
    pub airdrop: Account<'info, Airdrop>,

    /// A new SPL mint with decimals=9, authority = PDA (mint_authority)
    #[account(
        init,
        payer = authority,
        mint::decimals = 9,
        mint::authority = mint_authority
    )]
    pub mint: Account<'info, Mint>,

    /// The PDA used to sign future mint instructions
    #[account(
        seeds = [airdrop.key().as_ref(), MINT_AUTH_SEED],
        bump
    )]
    pub mint_authority: SystemAccount<'info>,

    /// The wallet paying for account creations
    #[account(mut)]
    pub authority: Signer<'info>,

    /// Programs
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

/// The user calls claim, referencing a pass that must be valid for the user & the airdrop’s gatekeeper network
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

    /// A "ticket" for uniqueness or preventing multiple claims
    #[account(
        init,
        payer = payer,
        seeds = [airdrop.key().as_ref(), recipient.key().as_ref(), TICKET_SEED],
        bump,
        space = Ticket::SIZE
    )]
    pub ticket: Account<'info, Ticket>,

    /// The minted token
    #[account(mut)]
    pub mint: Account<'info, Mint>,

    /// The user’s associated token account
    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = mint,
        associated_token::authority = recipient
    )]
    pub recipient_token_account: Account<'info, TokenAccount>,

    /// The pass must be valid for the `recipient` with the `gatekeeper_network`
    #[account(
        constraint = pass.valid(&recipient.key, &airdrop.gatekeeper_network)
            @ ErrorCode::InvalidPass
    )]
    pub pass: Account<'info, Pass>,

    /// The user receiving tokens
    #[account(mut)]
    pub recipient: SystemAccount<'info>,

    // Programs
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

/// Add a new owner
#[derive(Accounts)]
pub struct AddOwner<'info> {
    #[account(mut)]
    pub airdrop: Account<'info, Airdrop>,
    #[account(mut)]
    pub signer: Signer<'info>,
}

/// Delete an existing owner
#[derive(Accounts)]
pub struct DeleteOwner<'info> {
    #[account(mut)]
    pub airdrop: Account<'info, Airdrop>,
    #[account(mut)]
    pub signer: Signer<'info>,
}

/// Change gateway network
#[derive(Accounts)]
pub struct ChangeGateway<'info> {
    #[account(mut)]
    pub airdrop: Account<'info, Airdrop>,
    #[account(mut)]
    pub signer: Signer<'info>,
}

// -------------------------------------------------------------------
// DATA ACCOUNTS
// -------------------------------------------------------------------
#[account]
#[derive(Default)]
pub struct Airdrop {
    /// The gatekeeper network for Civic
    pub gatekeeper_network: Pubkey,

    /// The SPL Mint created at Initialize
    pub mint: Pubkey,

    /// Hard-coded daily amount
    pub daily_amount: u64,

    /// When the user last claimed
    pub last_claim_timestamp: i64,

    /// Ownership array
    pub owners: [Pubkey; 6],
    pub owners_count: u8,

    /// Freeze re-init
    pub initialized: bool,
}
impl Airdrop {
    pub const SIZE: usize = 300; // enough for fields + overhead
}

/// Minimal "ticket" to track unique user claims
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

    #[msg("Could not parse gatekeeper network as valid Pubkey")]
    InvalidPubkey,

    #[msg("Airdrop is already initialized")]
    AlreadyInitialized,
}

// -------------------------------------------------------------------
// HELPER LOGIC
// -------------------------------------------------------------------
fn is_authorized(signer_pubkey: &Pubkey, airdrop: &Airdrop) -> bool {
    for i in 0..airdrop.owners_count {
        if airdrop.owners[i as usize] == *signer_pubkey {
            return true;
        }
    }
    false
}

// Add owner
fn add_owner_logic(ctx: Context<AddOwner>, new_owner: Pubkey) -> Result<()> {
    let airdrop = &mut ctx.accounts.airdrop;
    let signer_key = ctx.accounts.signer.key();

    // Must be an existing owner
    require!(is_authorized(&signer_key, airdrop), ErrorCode::Unauthorized);

    // Ensure space
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

    // Insert new owner
    let idx = airdrop.owners_count as usize;
    airdrop.owners[idx] = new_owner;
    airdrop.owners_count += 1;
    msg!("Added new owner: {}", new_owner);
    Ok(())
}

// Delete owner
fn delete_owner_logic(ctx: Context<DeleteOwner>, target_owner: Pubkey) -> Result<()> {
    let airdrop = &mut ctx.accounts.airdrop;
    let signer_key = ctx.accounts.signer.key();

    require!(is_authorized(&signer_key, airdrop), ErrorCode::Unauthorized);

    if target_owner == signer_key {
        return err!(ErrorCode::CannotRemoveSelf);
    }

    // find the target in owners
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

    // swap remove
    let last_idx = airdrop.owners_count as usize - 1;
    if idx != last_idx {
        airdrop.owners[idx] = airdrop.owners[last_idx];
    }
    airdrop.owners[last_idx] = Pubkey::default();
    airdrop.owners_count -= 1;

    msg!("Deleted owner: {}", target_owner);
    Ok(())
}

// Change gateway network
fn change_gateway_logic(ctx: Context<ChangeGateway>, new_gatekeeper: Pubkey) -> Result<()> {
    let airdrop = &mut ctx.accounts.airdrop;
    let signer_key = ctx.accounts.signer.key();
    require!(is_authorized(&signer_key, airdrop), ErrorCode::Unauthorized);

    airdrop.gatekeeper_network = new_gatekeeper;
    msg!("Changed gatekeeper network to {}", new_gatekeeper);
    Ok(())
}
