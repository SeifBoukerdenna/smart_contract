use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

declare_id!("DNsprXHccVbxFTE2RNvchU3E3W1Hn3U4yosFSiVs8bQT");

#[program]
pub mod mcga_pool {
    use super::*;

    pub fn initialize_pool(ctx: Context<InitializePool>) -> Result<()> {
        let pool = &mut ctx.accounts.pool;
        pool.authority = ctx.accounts.authority.key();
        pool.token_account = ctx.accounts.pool_token_account.key();
        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        // Transfer tokens from user to pool
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
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializePool<'info> {
    #[account(init, payer = authority, space = 8 + 32 + 32)]
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
    #[account(mut)]
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
}
