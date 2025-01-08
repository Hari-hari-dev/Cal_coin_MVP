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
use std::str::FromStr;

// Replace with your actual Program ID
declare_id!("ADM5ikM5LS1ptrwFqXNyZDYazDzThSJknLNpJyw1x6c");

// Seeds used for PDAs
pub const TICKET_SEED: &[u8] = b"ticket";
pub const MINT_AUTH_SEED: &[u8] = b"mint_authority";

// -------------------------------------------------------------------
// PROGRAM
// -------------------------------------------------------------------
#[program]
pub mod daily_facescan {
    use super::*;

    /// (1) Initialize: Creates the Airdrop account & a new SPL Mint.
    /// Hard-coded `daily_amount = 1440`.
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let data = &mut ctx.accounts.airdrop;

        // 1) Check if already initialized
        if data.initialized {
            msg!("Airdrop is already initialized; aborting.");
            return err!(ErrorCode::AlreadyInitialized);
        }

        // 2) Optionally set a gatekeeper network if you want a reference
        //    or remove it if you don’t use Civic gating. Hard-coded example:
        let fixed_gatekeeper_network = Pubkey::from_str("uniqobk8oGh4XBLMqM68K8M2zNu3CdYX7q5go7whQiv")
            .map_err(|_| error!(ErrorCode::InvalidPubkey))?;
        data.gatekeeper_network = fixed_gatekeeper_network;

        data.mint = ctx.accounts.mint.key();
        data.daily_amount = 1440;
        data.last_claim_timestamp = 0;

        // Initialize owners array; we’ll store the `authority` as first owner
        data.owners[0] = ctx.accounts.authority.key();
        data.owners_count = 1;
        for i in 1..data.owners.len() {
            data.owners[i] = Pubkey::default();
        }

        // Mark as initialized
        data.initialized = true;
        Ok(())
    }

    /// (2) Claim: time-based daily logic.  
    /// No `gateway_token` param; we rely on the `payer` signature as the gating factor.
    pub fn claim(ctx: Context<Claim>) -> Result<()> {
        let data = &mut ctx.accounts.airdrop;

        // (Optional) Check if `payer` is authorized in some way.
        // For instance, if you want only “owners” to claim:
        /*
        if !is_authorized(&ctx.accounts.payer.key(), data) {
            msg!("Signer not authorized to claim");
            return err!(ErrorCode::Unauthorized);
        }
        */

        // 1) Time-based daily logic
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

        // 2) Mint if minted_amount > 0
        if minted_amount > 0 {
            // Derive the PDA seeds for the mint authority
            let airdrop_key = data.key();
            let seeds = &[
                airdrop_key.as_ref(),
                MINT_AUTH_SEED,
                &[ctx.bumps.mint_authority],
            ];
            let signer_seeds = &[&seeds[..]];

            // Use `token::mint_to` to mint into the user’s associated token account
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

    /// (3) Add a new owner if space is available.
    pub fn add_owner(ctx: Context<AddOwner>, new_owner: Pubkey) -> Result<()> {
        add_owner_logic(ctx, new_owner)
    }

    /// (4) Delete an existing owner.
    pub fn delete_owner(ctx: Context<DeleteOwner>, target_owner: Pubkey) -> Result<()> {
        delete_owner_logic(ctx, target_owner)
    }

    /// (5) Change gatekeeper network if the signer is an existing owner.
    pub fn change_gateway_network(ctx: Context<ChangeGateway>, new_gatekeeper: Pubkey) -> Result<()> {
        change_gateway_logic(ctx, new_gatekeeper)
    }
}

// -------------------------------------------------------------------
// ACCOUNTS
// -------------------------------------------------------------------
#[derive(Accounts)]
pub struct Initialize<'info> {
    /// Airdrop account, storing the mint info, daily amount, etc.
    #[account(
        init,
        payer = authority,
        space = Airdrop::SIZE
    )]
    pub airdrop: Account<'info, Airdrop>,

    /// A new SPL Mint with decimals=9, authority = the “mint_authority” PDA
    #[account(
        init,
        payer = authority,
        mint::decimals = 9,
        mint::authority = mint_authority
    )]
    pub mint: Account<'info, Mint>,

    /// Derive the mint authority via seeds = [airdrop, "mint_authority"]
    #[account(
        seeds = [airdrop.key().as_ref(), MINT_AUTH_SEED],
        bump
    )]
    pub mint_authority: SystemAccount<'info>,

    /// The user paying for account creations
    #[account(mut)]
    pub authority: Signer<'info>,

    // Programs
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

/// The user claims daily minted tokens. 
/// They must be the “payer” (and can also be the one gating).
#[derive(Accounts)]
pub struct Claim<'info> {
    /// The airdrop config must point to the same `mint`
    #[account(has_one = mint)]
    pub airdrop: Account<'info, Airdrop>,

    /// The user paying for (and gating) the claim instruction
    #[account(mut)]
    pub payer: Signer<'info>,

    /// The “mint_authority” PDA
    #[account(
        seeds = [airdrop.key().as_ref(), MINT_AUTH_SEED],
        bump
    )]
    pub mint_authority: SystemAccount<'info>,

    /// A “ticket” for uniqueness, if needed.
    #[account(
        init,
        payer = payer,
        seeds = [airdrop.key().as_ref(), payer.key().as_ref(), TICKET_SEED],
        bump,
        space = Ticket::SIZE
    )]
    pub ticket: Account<'info, Ticket>,

    /// The minted SPL
    #[account(mut)]
    pub mint: Account<'info, Mint>,

    /// The user’s token account for receiving minted tokens
    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = mint,
        associated_token::authority = payer
    )]
    pub recipient_token_account: Account<'info, TokenAccount>,

    /// Programs
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

// -------------------------------
// Additional instructions
// -------------------------------
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
    pub gatekeeper_network: Pubkey, // If you want to reference a Civic network
    pub mint: Pubkey,               // The minted SPL
    pub daily_amount: u64,          // Hard-coded to 1440 in `initialize`
    pub last_claim_timestamp: i64,

    // Optional multi-owner array
    pub owners: [Pubkey; 6],
    pub owners_count: u8,

    // Freeze re-init
    pub initialized: bool,
}

impl Airdrop {
    /// Enough space to store the data + 8 for account discriminator. 
    /// We put 300 as a safe overhead if you want to add more fields later.
    pub const SIZE: usize = 300;
}

/// Ticket for tracking one claim or unique usage
#[account]
pub struct Ticket {}
impl Ticket {
    /// 8 bytes for the account discriminator, no fields
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

    #[msg("Could not parse gatekeeper network as valid")]
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

// Logic to add a new owner if space
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

// Logic to remove an owner
fn delete_owner_logic(ctx: Context<DeleteOwner>, target_owner: Pubkey) -> Result<()> {
    let ad = &mut ctx.accounts.airdrop;
    let signer_key = ctx.accounts.signer.key();

    require!(is_authorized(&signer_key, ad), ErrorCode::Unauthorized);

    if target_owner == signer_key {
        return err!(ErrorCode::CannotRemoveSelf);
    }

    // Find target in owners
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

    // Remove by swapping with last
    let last_idx = ad.owners_count as usize - 1;
    if idx != last_idx {
        ad.owners[idx] = ad.owners[last_idx];
    }
    ad.owners[last_idx] = Pubkey::default();
    ad.owners_count -= 1;

    msg!("Deleted owner: {}", target_owner);
    Ok(())
}

// Logic to change gateway network
fn change_gateway_logic(ctx: Context<ChangeGateway>, new_gatekeeper: Pubkey) -> Result<()> {
    let ad = &mut ctx.accounts.airdrop;
    let signer_key = ctx.accounts.signer.key();

    require!(is_authorized(&signer_key, ad), ErrorCode::Unauthorized);

    ad.gatekeeper_network = new_gatekeeper;
    msg!("Updated gatekeeper network => {}", new_gatekeeper);
    Ok(())
}
