use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::Token};

declare_id!("2sH2Q2kcFvWpxjXqv78THttyfVKS25eNFvuD3vaj6BiN");

#[program]
pub mod carbon_engine {
	use super::*;

	pub fn mint(ctx: Context<MintNft>) -> Result<()> {
		msg!("Creating mint account...");
		msg!("Mint: {}", &ctx.accounts.mint.key());

		msg!("Initializing mint account...");
		msg!("Mint: {}", &ctx.accounts.mint.key());

		msg!("Creating token account...");
		msg!("Token Address: {}", &ctx.accounts.token_account.key());

		msg!("Minting token to account...");
		msg!("Mint: {}", &ctx.accounts.mint.to_account_info().key());
		msg!("Token Address: {}", &ctx.accounts.token_account.key());

		msg!("Creating metadata acount...");
		msg!(
			"Metadata acount address: {}",
			&ctx.accounts.metadata.to_account_info().key(),
		);

		msg!("Create master edition metadata account...");
		msg!(
			"Master edition metadata acount address: {}",
			&ctx.accounts.master_edition.to_account_info().key(),
		);

		msg!("Token mint process completed successfully!");

		Ok(())
	}
}

#[derive(Accounts)]
pub struct MintNft<'info> {
	/// CHECK: This part handled by Metaplex
	#[account(mut)]
	pub metadata: UncheckedAccount<'info>,
	#[account(mut)]
	/// CHECK: We're about to create this with Metaplex
	pub master_edition: UncheckedAccount<'info>,
	#[account(mut)]
	pub mint: Signer<'info>,
	/// CHECK: We're about to create this with Anchor
	#[account(mut)]
	pub token_account: UncheckedAccount<'info>,
	#[account(mut)]
	pub mint_authority: Signer<'info>,
	pub rent: Sysvar<'info, Rent>,
	pub system_program: Program<'info, System>,
	pub token_program: Program<'info, Token>,
	pub associated_token_program: Program<'info, AssociatedToken>,
	/// CHECK: Metaplex will check this
	pub token_metadata_program: UncheckedAccount<'info>,
}
