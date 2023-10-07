use anchor_lang::{prelude::*, solana_program::program::invoke, system_program};
use anchor_spl::{associated_token, associated_token::AssociatedToken, token, token::Token};
use mpl_token_metadata::{instructions as token_instructions, ID as TOKEN_METADATA_ID};

declare_id!("2sH2Q2kcFvWpxjXqv78THttyfVKS25eNFvuD3vaj6BiN");

#[program]
pub mod carbon_engine {
	use super::*;

	pub fn mint(ctx: Context<MintNft>) -> Result<()> {
		msg!("Creating mint account...");
		msg!("Mint: {}", &ctx.accounts.mint.key());
		system_program::create_account(
			CpiContext::new(
				ctx.accounts.token_program.to_account_info(),
				system_program::CreateAccount {
					from: ctx.accounts.mint_authority.to_account_info(),
					to: ctx.accounts.mint.to_account_info(),
				},
			),
			10_000_000,
			82,
			&ctx.accounts.token_program.key(),
		)?;

		msg!("Initializing mint account...");
		msg!("Mint: {}", &ctx.accounts.mint.key());
		token::initialize_mint(
			CpiContext::new(
				ctx.accounts.token_program.to_account_info(),
				token::InitializeMint {
					mint: ctx.accounts.token_program.to_account_info(),
					rent: ctx.accounts.rent.to_account_info(),
				},
			),
			0,
			&ctx.accounts.mint_authority.key(),
			Some(&ctx.accounts.mint_authority.key()),
		)?;

		msg!("Creating token account...");
		msg!("Token Address: {}", &ctx.accounts.token_account.key());
		associated_token::create(CpiContext::new(
			ctx.accounts.associated_token_program.to_account_info(),
			associated_token::Create {
				payer: ctx.accounts.mint_authority.to_account_info(),
				associated_token: ctx.accounts.token_account.to_account_info(),
				authority: ctx.accounts.mint_authority.to_account_info(),
				mint: ctx.accounts.mint.to_account_info(),
				system_program: ctx.accounts.associated_token_program.to_account_info(),
				token_program: ctx.accounts.token_program.to_account_info(),
			},
		))?;

		msg!("Minting token to account...");
		msg!("Mint: {}", &ctx.accounts.mint.to_account_info().key());
		msg!("Token Address: {}", &ctx.accounts.token_account.key());
		token::mint_to(
			CpiContext::new(
				ctx.accounts.token_program.to_account_info(),
				token::MintTo {
					mint: ctx.accounts.mint.to_account_info(),
					to: ctx.accounts.token_account.to_account_info(),
					authority: ctx.accounts.associated_token_program.to_account_info(),
				},
			),
			1,
		)?;

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
