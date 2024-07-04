use crate::{
    check_health_factor, Collateral, Config, SEED_COLLATERAL_ACCOUNT, SEED_CONFIG_ACCOUNT,
    SEED_MINT_ACCOUNT,
};
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_2022::{mint_to, MintTo};
use anchor_spl::token_interface::{Mint, Token2022, TokenAccount};
use pyth_solana_receiver_sdk::price_update::PriceUpdateV2;

#[derive(Accounts)]
pub struct MintTokens<'info> {
    #[account(mut)]
    pub depositer: Signer<'info>,

    #[account(
        seeds = [SEED_CONFIG_ACCOUNT],
        bump = config_account.bump,
    )]
    pub config_account: Account<'info, Config>,
    #[account(
        mut,
        seeds = [SEED_COLLATERAL_ACCOUNT, depositer.key().as_ref()],
        bump = collateral_account.bump,
    )]
    pub collateral_account: Account<'info, Collateral>,
    #[account(
        mut,
        seeds = [SEED_MINT_ACCOUNT],
        bump = config_account.bump_mint_account,
        mint::token_program = token_program
    )]
    pub mint_account: InterfaceAccount<'info, Mint>,
    pub price_update: Account<'info, PriceUpdateV2>,
    #[account(
        init_if_needed,
        payer = depositer,
        associated_token::mint = mint_account,
        associated_token::authority = depositer,
        associated_token::token_program = token_program
    )]
    pub token_account: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Program<'info, Token2022>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn process_mint_tokens(ctx: Context<MintTokens>, amount_to_mint: u64) -> Result<()> {
    check_health_factor(
        &ctx.accounts.collateral_account,
        &ctx.accounts.config_account,
        &ctx.accounts.price_update,
    )?;

    let bump = ctx.accounts.config_account.bump_mint_account;
    let signer_seeds: &[&[&[u8]]] = &[&[SEED_MINT_ACCOUNT, &[bump]]];
    mint_to(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.mint_account.to_account_info(),
                to: ctx.accounts.token_account.to_account_info(),
                authority: ctx.accounts.mint_account.to_account_info(),
            },
        )
        .with_signer(signer_seeds),
        amount_to_mint,
    )?;

    let collateral_account = &mut ctx.accounts.collateral_account;
    collateral_account.amount_minted += amount_to_mint;

    Ok(())
}
