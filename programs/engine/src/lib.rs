use anchor_spl::token::TokenAccount;

use {
	anchor_lang::{prelude::*, solana_program::program::invoke, system_program},
	anchor_spl::{associated_token, associated_token::AssociatedToken, token, token::Token},
	mpl_token_metadata::{instruction as token_instruction, ID as TOKEN_METADATA_ID},
};

declare_id!("6vKoSG8vRdDbW5iZLyZU8tvWQE3981Dc65V3V85inQzY");

#[program]
pub mod carbon_engine {

	use super::*;
	use std::str::FromStr;

	pub fn mint(
		ctx: Context<MintNft>,
		metadata_title: String,
		metadata_symbol: String,
		metadata_uri: String,
	) -> Result<()> {
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
					mint: ctx.accounts.mint.to_account_info(),
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
				system_program: ctx.accounts.system_program.to_account_info(),
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
					authority: ctx.accounts.mint_authority.to_account_info(),
				},
			),
			1,
		)?;

		msg!("Creating metadata account...");
		msg!(
			"Metadata account address: {}",
			&ctx.accounts.metadata.to_account_info().key(),
		);
		invoke(
			&token_instruction::create_metadata_accounts_v3(
				TOKEN_METADATA_ID,
				ctx.accounts.metadata.key(),
				ctx.accounts.mint.key(),
				ctx.accounts.mint_authority.key(),
				ctx.accounts.mint_authority.key(),
				ctx.accounts.mint_authority.key(),
				metadata_title,
				metadata_symbol,
				metadata_uri,
				None,
				1,
				true,
				false,
				None,
				None,
				None,
			),
			&[
				ctx.accounts.metadata.to_account_info(),
				ctx.accounts.mint.to_account_info(),
				ctx.accounts.token_account.to_account_info(),
				ctx.accounts.mint_authority.to_account_info(),
				ctx.accounts.rent.to_account_info(),
			],
		)?;

		msg!("Token mint process completed successfully!");

		Ok(())
	}

	pub fn test_mint(ctx: Context<CalculatorContext>, num1: i64) -> Result<()> {
		let pubkey_collection =
			[Pubkey::from_str("9E5khVvUyyuny6MNL7C7aMbMTMUBwC27dr9WC947Di68").unwrap()];
		msg!(
			"The key is {}",
			pubkey_collection.contains(&ctx.accounts.signer.key())
		);
		require!(
			pubkey_collection.contains(&ctx.accounts.signer.key()),
			CarbonEnginError::UnauthorizedAccount
		);
		let calculator = &mut ctx.accounts.calculator_data;
		calculator.data = num1;

		Ok(())
	}

	pub fn create_master(ctx: Context<CreateMaster>) -> Result<()> {
		msg!("Creating master edition metadata account...");
		msg!(
			"Master edition metadata account address: {}",
			&ctx.accounts.master_edition.to_account_info().key()
		);
		invoke(
			&token_instruction::create_master_edition_v3(
				TOKEN_METADATA_ID,
				ctx.accounts.master_edition.key(),
				ctx.accounts.mint.key(),
				ctx.accounts.mint_authority.key(),
				ctx.accounts.mint_authority.key(),
				ctx.accounts.metadata.key(),
				ctx.accounts.mint_authority.key(),
				Some(0),
			),
			&[
				ctx.accounts.master_edition.to_account_info(),
				ctx.accounts.metadata.to_account_info(),
				ctx.accounts.mint.to_account_info(),
				ctx.accounts.token_account.to_account_info(),
				ctx.accounts.mint_authority.to_account_info(),
				ctx.accounts.rent.to_account_info(),
			],
		)?;
		Ok(())
	}
}

#[derive(Accounts)]
pub struct MintNft<'info> {
	/// CHECK: This part handled by Metaplex
	#[account(mut)]
	pub metadata: UncheckedAccount<'info>,

	/// CHECK: We're about to create this with Metaplex
	#[account(mut)]
	pub mint: Signer<'info>,

	/// CHECK: We're about to create this with Anchor
	#[account(mut)]
	pub token_account: UncheckedAccount<'info>,

	#[account(mut)]
	pub mint_authority: Signer<'info>,

	#[account(mut)]
	pub payer: Signer<'info>,

	pub rent: Sysvar<'info, Rent>,
	pub system_program: Program<'info, System>,
	pub token_program: Program<'info, Token>,
	pub associated_token_program: Program<'info, AssociatedToken>,
	/// CHECK: Metaplex will check this
	pub token_metadata_program: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct CreateMaster<'info> {
	/// CHECK: This part handled by Metaplex
	#[account(mut)]
	pub master_edition: UncheckedAccount<'info>,

	/// CHECK:
	#[account(mut)]
	pub metadata: UncheckedAccount<'info>,

	#[account()]
	pub token_account: Account<'info, TokenAccount>,

	/// CHECK:
	#[account(mut)]
	pub mint: UncheckedAccount<'info>,

	#[account()]
	pub mint_authority: Signer<'info>,

	pub rent: Sysvar<'info, Rent>,
	pub system_program: Program<'info, System>,
	pub token_program: Program<'info, Token>,
	pub associated_token_program: Program<'info, AssociatedToken>,
	/// CHECK: Metaplex will check this
	pub token_metadata_program: UncheckedAccount<'info>,
}

#[account]
pub struct Calculator {
	data: i64,
}

#[derive(Accounts)]
pub struct CalculatorContext<'info> {
	#[account(init, payer=signer, space=264)]
	pub calculator_data: Account<'info, Calculator>,

	#[account(mut)]
	pub signer: Signer<'info>,
	pub system_program: Program<'info, System>,
}

#[error_code]
pub enum CarbonEnginError {
	#[msg("The signer is unauthorized")]
	UnauthorizedAccount,
}
