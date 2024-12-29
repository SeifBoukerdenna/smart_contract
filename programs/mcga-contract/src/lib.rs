use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

declare_id!("DNsprXHccVbxFTE2RNvchU3E3W1Hn3U4yosFSiVs8bQT");

#[program]
pub mod mcga_pool {
    use super::*;

    pub fn initialize_pool(ctx: Context<InitializePool>, seed: String, secret_hash: String) -> Result<()> {
        let pool = &mut ctx.accounts.pool;
        pool.authority = ctx.accounts.authority.key();
        pool.token_account = ctx.accounts.pool_token_account.key();
        pool.secret_hash = secret_hash;
        pool.seed = seed;
        Ok(())
    }

    // First step: Just deposit tokens
    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        let transfer_to_pool = Transfer {
            from: ctx.accounts.user_token_account.to_account_info(),
            to: ctx.accounts.pool_token_account.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };

        let cpi_ctx_to_pool = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            transfer_to_pool,
        );

        token::transfer(cpi_ctx_to_pool, amount)?;
        Ok(())
    }

    // Second step: Check hash and potentially win pool
    pub fn check_hash(ctx: Context<Deposit>, attempt_hash: String) -> Result<()> {
        let pool = &ctx.accounts.pool;

        // Only transfer if hash matches
        if attempt_hash == pool.secret_hash {
            let pool_balance = ctx.accounts.pool_token_account.amount;

            let transfer_to_user = Transfer {
                from: ctx.accounts.pool_token_account.to_account_info(),
                to: ctx.accounts.user_token_account.to_account_info(),
                authority: ctx.accounts.pool.to_account_info(),
            };

            let seeds = &[
                pool.seed.as_bytes(),
                &[ctx.bumps.pool],
            ];
            let signer = &[&seeds[..]];

            let cpi_ctx_to_user = CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                transfer_to_user,
                signer,
            );

            token::transfer(cpi_ctx_to_user, pool_balance)?;
        }

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(seed: String, secret_hash: String)]
pub struct InitializePool<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 32 + 64 + 64 + 8,
        seeds = [seed.as_bytes()],
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
        seeds = [pool.seed.as_bytes()],
        bump
    )]
    pub pool: Account<'info, Pool>,
    #[account(
        mut,
        constraint = pool_token_account.key() == pool.token_account
    )]
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
    pub secret_hash: String,
    pub seed: String,
}