use anchor_lang::prelude::*;

declare_id!("2sH2Q2kcFvWpxjXqv78THttyfVKS25eNFvuD3vaj6BiN");

#[program]
pub mod carbon_contract {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
