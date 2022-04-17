use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod dynamic_nft_marketplace {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>, nft_address: Pubkey) -> Result<()> {
        let nft_data = &mut ctx.accounts.nft_data;
        nft_data.authority = *ctx.accounts.user.key;
        nft_data.sold = false;
        nft_data.nft_address = nft_address;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {
    #[account(init, payer = user, space = 8 + 32 + 1 + 32)]
    pub nft_data: Account<'info, nft>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}


#[account]
pub struct nft {
    pub authority: Pubkey,
    pub sold: bool,
    pub nft_address: Pubkey,
}
