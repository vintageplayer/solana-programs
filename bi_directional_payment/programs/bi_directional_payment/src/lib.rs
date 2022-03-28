use anchor_lang::solana_program::instruction::Instruction;
use anchor_lang::prelude::*;

declare_id!("8bFwN63CagjrFrVtymRBrpNj5RtfibdJTNvJA8bBzomo");

#[program]
pub mod bi_directional_payment {
    use super::*;
    pub fn create_channel(
        ctx: Context<CreateChannel>,
        users: [Pubkey; 2],
        balances: [u32; 2],
        expires_at: u32,
        challenge_period: u32
        ) -> Result<()> {

        let channel = &mut ctx.accounts.channel;

        require!(users[0] != users[1], ChannelError::DuplicateUsers);
        require!(challenge_period > 0, ChannelError::InvalidChallengePeriod);

        // TO DO: Fix anchor failing to parse i64 using casting 
        // require!(expires_at > Clock::get().unwrap().unix_timestamp, ChannelError::InvalidExpiryDate);

        channel.users = users;
        channel.balances = balances;
        channel.expires_at = expires_at as i64;
        channel.challenge_period = challenge_period;
        channel.nonce = 0;
        Ok(())
    }

    pub fn challenge_exit(
        ctx: Context<ChallengeExit>,
        balances: [u32; 2],
        nonce: u8
        ) -> Result<()> {
        let user1 = ctx.accounts.user1.key;
        let user2 = ctx.accounts.user1.key;

        // TO DO: Fix anchor failing to parse i64 using casting 
        // require!(ctx.accounts.channel.expires_at > Clock::get().unwrap().unix_timestamp, ChannelError::ExpiryPending);

        let user_1_index = ctx.accounts.channel.users.iter().position(|a| a == user1).ok_or(ChannelError::InvalidUser)?;
        let user_2_index = ctx.accounts.channel.users.iter().position(|a| a == user2).ok_or(ChannelError::InvalidUser)?;
        require!(nonce > ctx.accounts.channel.nonce, ChannelError::InvalidNonce);
        let channel = &mut ctx.accounts.channel;
        channel.nonce = nonce;
        channel.balances = balances;
        channel.expires_at = Clock::get().unwrap().unix_timestamp + (channel.challenge_period as i64);
        Ok(())
    }

    pub fn withdraw(
        ctx: Context<Withdraw>
        ) -> Result<()> {
        let user = ctx.accounts.user.key;

        // require!(ctx.accounts.channel.expires_at <= Clock::get().unwrap().unix_timestamp, ChannelError::ExpiryPending);

        let user_index = ctx.accounts.channel.users.iter().position(|a| a == user).ok_or(ChannelError::InvalidUser)?;
        let amount = ctx.accounts.channel.balances[user_index];
        msg!("{:?}", amount);

        // TO DO: Write code to send balance to user

        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.channel.key(),
            &ctx.accounts.user.key(),
            amount.into(),
        );

        let channel = &mut ctx.accounts.channel;
        channel.balances[user_index] = 0;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateChannel<'info> {
    #[account(init, payer=user, space = 8+64+16+16)]
    pub channel: Account<'info, Channel>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ChallengeExit<'info> {
    #[account(mut)]
    pub channel: Account<'info, Channel>,
    pub user1: Signer<'info>,
    pub user2: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub channel: Account<'info, Channel>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Channel {
    pub users: [Pubkey; 2],
    pub balances: [u32; 2],
    pub expires_at: i64,
    pub challenge_period: u32,
    pub nonce: u8,
}

#[error_code]
pub enum ChannelError {
    #[msg("Duplicate PubKey given for users!")]
    DuplicateUsers,
    #[msg("Challenge Period has to be greater than 0!")]
    InvalidChallengePeriod,
    #[msg("Expiry Time needs to be in future!")]
    InvalidExpiryDate,
    #[msg("Can't Withdraw before expiry of challenge window!")]
    ExpiryPending,
    #[msg("Can only be invoked by a channel user!")]
    InvalidUser,
    #[msg("Nonce has to be higher than previous recorded nonce!")]
    InvalidNonce
}