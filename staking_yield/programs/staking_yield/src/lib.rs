use anchor_lang::prelude::*;
use anchor_spl::token::{self, MintTo, Transfer};
use crate::errors::StakingErrors;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod staking_yield {
    use super::*;
    pub fn mint_to(ctx: Context<ProxyMintTo>, amount: u64) -> Result<()> {
        token::mint_to(ctx.accounts.into(), amount)
    }

    pub fn deposit_stake(ctx: Context<Deposit>, amount: u64, end_date: u128) -> Result<()> {
        token::transfer(ctx.accounts.into(), amount)?;

        let stake = &mut ctx.accounts.stake;
        stake.staker = *ctx.accounts.staker.key;
        stake.amount = amount;
        stake.end_time = end_date;

        Ok(())
    }

    pub fn withdraw_stake(ctx: Context<Withdraw>) -> Result<()> {
        // Check if end time is reached
        if ctx.accounts.stake.end_time > Clock::get().unwrap().unix_timestamp as u128 {
            return Err(error!(Errors::StakeTimeNotOver));
        }

        // Provide with an interest of 10%
        let interest_amount = ctx.accounts.stake.amount as u64 * 1 as u64 / 10;

        token::mint_to(ctx.accounts.into(), interest_amount)?;

        // Return the staked amount
        let amount = ctx.accounts.stake.amount;
        token::transfer(ctx.accounts.into(), amount)?;

        let stake = &mut ctx.accounts.stake;
        stake.amount = 0;
        stake.end_time = 0;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct ProxyMintTo<'info> {
    #[account(signer)]
    pub authority: AccountInfo<'info>,
    #[account(mut)]
    pub to: AccountInfo<'info>,
    #[account(mut)]
    pub mint: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
}

impl<'a, 'b, 'c, 'info> From<&mut ProxyMintTo<'info>>
    for CpiContext<'a, 'b, 'c, 'info, MintTo<'info>>
{
    fn from(accounts: &mut ProxyMintTo<'info>) -> CpiContext<'a, 'b, 'c, 'info, MintTo<'info>> {
        let cpi_accounts = MintTo {
            to: accounts.to.clone(),
            authority: accounts.authority.clone(),
            mint: accounts.mint.clone(),
        };

        let cpi_program = accounts.token_program.clone();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(signer, mut)]
    pub authority: AccountInfo<'info>, // mint authority
    #[account(mut)]
    pub mint: AccountInfo<'info>, // mint account
    #[account(mut)]
    pub staker: AccountInfo<'info>, // stakers associated token account
    #[account(init, payer = authority, space = 8 + 8 + 16 + 32)]
    pub stake: Account<'info, Stake>, // stake state account
    #[account(mut)]
    pub stake_vault: AccountInfo<'info>, // account to hold stake amount
    pub system_program: Program<'info, System>,
    pub token_program: AccountInfo<'info>,
}

impl<'a, 'b, 'c, 'info> From<&mut CreateStake<'info>>
    for CpiContext<'a, 'b, 'c, 'info, Transfer<'info>>
{
    fn from(accounts: &mut CreateStake<'info>) -> CpiContext<'a, 'b, 'c, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            authority: accounts.authority.clone(),
            from: accounts.staker.clone(),
            to: accounts.stake_vault.clone(),
        };

        CpiContext::new(accounts.token_program.clone(), cpi_accounts)
    }
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(signer)]
    pub authority: AccountInfo<'info>,
    #[account(mut)]
    pub mint: AccountInfo<'info>,
    #[account(mut)]
    pub stake_vault: AccountInfo<'info>,
    #[account(mut)]
    pub stake: Account<'info, Stake>,
    #[account(mut)]
    pub staker: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
}

impl<'a, 'b, 'c, 'info> From<&mut EndStake<'info>>
    for CpiContext<'a, 'b, 'c, 'info, Transfer<'info>>
{
    fn from(accounts: &mut EndStake<'info>) -> CpiContext<'a, 'b, 'c, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            authority: accounts.authority.clone(),
            from: accounts.stake_vault.clone(),
            to: accounts.staker.clone(),
        };

        CpiContext::new(accounts.token_program.clone(), cpi_accounts)
    }
}

impl<'a, 'b, 'c, 'info> From<&mut EndStake<'info>>
    for CpiContext<'a, 'b, 'c, 'info, MintTo<'info>>
{
    fn from(accounts: &mut EndStake<'info>) -> CpiContext<'a, 'b, 'c, 'info, MintTo<'info>> {
        let cpi_accounts = MintTo {
            to: accounts.staker.clone(),
            authority: accounts.authority.clone(),
            mint: accounts.mint.clone(),
        };

        let cpi_program = accounts.token_program.clone();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

#[account]
pub struct Stake {
    pub amount: u64,
    pub end_time: u128,
    pub staker: Pubkey,
}

#[error_code]
pub enum Errors {
    #[msg("Can't Withdraw Before Stake Duration is over!!")]
    StakeTimeNotOver,
}