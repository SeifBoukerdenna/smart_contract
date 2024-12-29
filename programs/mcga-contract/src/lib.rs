use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use sha2::{Sha256, Digest};

declare_id!("DNsprXHccVbxFTE2RNvchU3E3W1Hn3U4yosFSiVs8bQT");

#[program]
pub mod mcga_pool {
    use super::*;

    // The secret hash for the prize (in practice, this would be set during initialization)
    const SECRET_HASH: &str = "5e884898da28047151d0e56f8dc6292773603d0d6aabbdd62a11ef721d1542d8"; // This is the hash of "password"

    pub fn initialize_pool(ctx: Context<InitializePool>) -> Result<()> {
        let pool = &mut ctx.accounts.pool;
        pool.authority = ctx.accounts.authority.key();
        pool.token_account = ctx.accounts.pool_token_account.key();
        Ok(())
    }

    pub fn deposit_with_secret(ctx: Context<Deposit>, amount: u64, secret: String) -> Result<()> {
        // Hash the provided secret
        let mut hasher = Sha256::new();
        hasher.update(secret.as_bytes());
        let result = format!("{:x}", hasher.finalize());

        if result == SECRET_HASH {
            // Secret matches - transfer all pool tokens to the user
            let pool_amount = ctx.accounts.pool_token_account.amount;

            let transfer_instruction = Transfer {
                from: ctx.accounts.pool_token_account.to_account_info(),
                to: ctx.accounts.user_token_account.to_account_info(),
                authority: ctx.accounts.pool.to_account_info(),
            };

            let seeds = &[
                b"pool".as_ref(),
                &[ctx.accounts.pool.bump],
            ];

            let signer_seeds = &[&seeds[..]];

            let cpi_ctx = CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                transfer_instruction,
                signer_seeds,
            );

            token::transfer(cpi_ctx, pool_amount)?;
        } else {
            // Wrong secret - transfer user tokens to pool
            let transfer_instruction = Transfer {
                from: ctx.accounts.user_token_account.to_account_info(),
                to: ctx.accounts.pool_token_account.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            };

            let cpi_ctx = CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                transfer_instruction,
            );

            token::transfer(cpi_ctx, amount)?;
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializePool<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 32 + 1, // Added 1 for bump
        seeds = [b"pool"],
        bump
    )]
    pub pool: Account<'info, Pool>,
    #[account(
        init,
        payer = authority,
        token::mint = mcga_mint,
        token::authority = pool,
    )]
    pub pool_token_account: Account<'info, TokenAccount>,
    pub mcga_mint: Account<'info, token::Mint>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(
        mut,
        seeds = [b"pool"],
        bump,
    )]
    pub pool: Account<'info, Pool>,
    #[account(mut)]
    pub pool_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,
    pub user: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct Pool {
    pub authority: Pubkey,
    pub token_account: Pubkey,
    pub bump: u8,
}