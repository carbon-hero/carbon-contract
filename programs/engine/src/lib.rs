use {
    anchor_lang::{
        prelude::*,
        solana_program::program::invoke,
        system_program,
    },
    anchor_spl::{
        associated_token,
        associated_token::AssociatedToken,
        token,
        token::Token

    },
    mpl_token_metadata::{
        ID as TOKEN_METADATA_ID,
        instruction as token_instruction,
        
    }
};

declare_id!("5fEoAvRHiDtE1UCEH56s8Jp8xbnmrVZGZYM4Tkub2K44");

#[program]
pub mod carbon_engine {
    use std::str::FromStr;
    use super::*;

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

		msg!("Creating metadata acount...");
		msg!(
			"Metadata acount address: {}",
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

		// msg!("Create master edition metadata account...");
		// msg!(
		// 	"Master edition metadata acount address: {}",
		// 	&ctx.accounts.master_edition.to_account_info().key(),
		// );

		msg!("Token mint process completed successfully!");

		Ok(())
	}
}

#[derive(Accounts)]
pub struct MintNft<'info> {
	/// CHECK: This part handled by Metaplex
	#[account(mut)]
	pub metadata: UncheckedAccount<'info>,
	/// CHECK: We're about to create this with Metaplex
	// #[account(mut)]
	// pub master_edition: UncheckedAccount<'info>,
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
    UnauthorizedAccount
}