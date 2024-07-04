use crate::{
    check_health_factor, mint_tokens_internal, Collateral, Config, SEED_COLLATERAL_ACCOUNT,
    SEED_CONFIG_ACCOUNT,
};
use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, Token2022, TokenAccount};
use pyth_solana_receiver_sdk::price_update::PriceUpdateV2;

#[derive(Accounts)]
pub struct MintTokens<'info> {
    #[account(mut)]
    pub depositor: Signer<'info>,

    #[account(
        seeds = [SEED_CONFIG_ACCOUNT],
        bump = config_account.bump,
        has_one = mint_account
    )]
    pub config_account: Account<'info, Config>,
    #[account(
        mut,
        seeds = [SEED_COLLATERAL_ACCOUNT, depositor.key().as_ref()],
        bump = collateral_account.bump,
        has_one = depositor,
        has_one = token_account
    )]
    pub collateral_account: Account<'info, Collateral>,
    #[account(mut)]
    pub mint_account: InterfaceAccount<'info, Mint>,
    pub price_update: Account<'info, PriceUpdateV2>,
    #[account(mut)]
    pub token_account: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
}

// https://github.com/Cyfrin/foundry-defi-stablecoin-cu/blob/main/src/DSCEngine.sol#L255
pub fn process_mint_tokens(ctx: Context<MintTokens>, amount_to_mint: u64) -> Result<()> {
    check_health_factor(
        &ctx.accounts.collateral_account,
        &ctx.accounts.config_account,
        &ctx.accounts.price_update,
    )?;

    mint_tokens_internal(
        &ctx.accounts.mint_account,
        &ctx.accounts.token_account,
        &ctx.accounts.token_program,
        ctx.accounts.config_account.bump_mint_account,
        amount_to_mint,
    )?;

    let collateral_account = &mut ctx.accounts.collateral_account;
    collateral_account.amount_minted += amount_to_mint;

    Ok(())
}
