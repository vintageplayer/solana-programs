use anchor_lang::prelude::*;
use std::vec::Vec;

declare_id!("YCGYEbSaZ9NHAMu3hk6hgEuSHtxd81d1EBhbUYhGDB5");

#[program]
pub mod voting {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>, proposals: Vec<String>, approved_accounts: Vec<Pubkey>) -> Result<()> {
        let proposal_votes = &mut ctx.accounts.proposal_votes;
        proposal_votes.proposals = proposals;
        let num_of_proposals = proposal_votes.proposals.len();
        let votes = vec![0; num_of_proposals];
        proposal_votes.vote_count = votes;

        let whitelist = &mut ctx.accounts.voters;
        whitelist.allowed_accounts = approved_accounts;
        let num_of_voters = whitelist.allowed_accounts.len();
        let is_voted = vec![false; num_of_voters];
        whitelist.voted = is_voted;
        Ok(())
    }

    pub fn vote_for_proposal(ctx: Context<CastVote>, proposal: String) -> Result<()> {
        let proposal_votes = &mut ctx.accounts.proposal_votes;
        let voters = &mut ctx.accounts.voters;
        let voter = &mut ctx.accounts.voter;
        let voter_pub_key = voter.to_account_info().key();

        // TO DO: Do Better Error handling than unwrap
        let proposal_index = proposal_votes.proposals.iter().position(|r| *r == proposal).unwrap();
        let voter_index = voters.allowed_accounts.iter().position(|&r| r == voter_pub_key).unwrap();
        if voters.voted[voter_index] == false {
            voters.voted[voter_index] = true;
            proposal_votes.vote_count[proposal_index] += 1;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 16 + 80)]
    pub proposal_votes: Account<'info, ProposalVotes>,

    #[account(init, payer = user, space = 16 + 80)]
    pub voters: Account<'info, AccountWhitelist>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CastVote<'info> {
    #[account(mut)]
    pub proposal_votes: Account<'info, ProposalVotes>,
    pub voters: Account<'info, AccountWhitelist>,
    pub voter: Signer<'info>
}

#[account]
pub struct AccountWhitelist {
    pub allowed_accounts: Vec<Pubkey>,
    pub voted: Vec<bool>
}

#[account]
pub struct ProposalVotes {
    pub proposals: Vec<String>,
    pub vote_count: Vec<u64>,
}