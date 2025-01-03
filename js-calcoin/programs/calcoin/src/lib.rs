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
use anchor_lang::pubkey;  // <- import the macro here


declare_id!("5nghtxRD9idNc6CmEP73AKrpoQBpWjt8qsVQFqWRsgD8");

// Seeds
pub const TICKET_SEED: &[u8] = b"ticket";
pub const MINT_AUTH_SEED: &[u8] = b"mint_authority";

// -------------------------------------------------------------------
// PROGRAM
// -------------------------------------------------------------------
#[program]
pub mod daily_facescan {
    use super::*;

    // ---------------------------------------------------------------
    // (1) Initialize: Creates the Airdrop account & a new SPL Mint
    // ---------------------------------------------------------------
    // No arguments for the user to pass in. Hard-coded daily_amount = 1440
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let data = &mut ctx.accounts.airdrop;

        // Hard-coded Gatekeeper Network
        let fixed_gatekeeper_network = pubkey!("uniqobk8oGh4XBLMqM68K8M2zNu3CdYX7q5go7whQiv");

        data.authority = ctx.accounts.authority.key();
        data.gatekeeper_network = fixed_gatekeeper_network;
        data.mint = ctx.accounts.mint.key();  // store newly created Mint
        data.daily_amount = 1440;             // <--- HARD-CODED
        data.last_claim_timestamp = 0;

        Ok(())
    }

    // ---------------------------------------------------------------
    // (2) Claim
    // ---------------------------------------------------------------
    pub fn claim(ctx: Context<Claim>) -> Result<()> {
        let data = &mut ctx.accounts.airdrop;

        // 1) Gateway check
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
        // cap at 7 days
        if delta > 7 * 86400 {
            delta = 7 * 86400;
        }
        let tokens_per_second = data.daily_amount as f64 / 86400.0; // 1440 / 86400
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

    // Ticket for uniqueness/tracking, if needed
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

    // The user’s token account for receiving tokens
    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = mint,
        associated_token::authority = recipient
    )]
    pub recipient_token_account: Account<'info, TokenAccount>,

    /// CHECK: verified at runtime via solana_gateway
    pub gateway_token: UncheckedAccount<'info>,

    #[account(mut)]
    pub recipient: SystemAccount<'info>,

    // Programs
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

// -------------------------------------------------------------------
// DATA
// -------------------------------------------------------------------
#[account]
#[derive(Default)]
pub struct Airdrop {
    pub authority: Pubkey,
    pub gatekeeper_network: Pubkey,   // For solana-gateway
    pub mint: Pubkey,                 // The newly created Mint
    pub daily_amount: u64,            // Hard-coded to 1440 in `initialize`
    pub last_claim_timestamp: i64,
}
impl Airdrop {
    pub const SIZE: usize = 8  // discriminator
        + 32  // authority
        + 32  // gatekeeper_network
        + 32  // mint
        + 8   // daily_amount
        + 8;  // last_claim_timestamp
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
}
