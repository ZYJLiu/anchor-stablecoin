use crate::{
    check_health_factor, Collateral, Config, SEED_COLLATERAL_ACCOUNT, SEED_CONFIG_ACCOUNT,
    SEED_MINT_ACCOUNT,
};
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_2022::{burn, Burn};
use anchor_spl::token_interface::{Mint, Token2022, TokenAccount};
use pyth_solana_receiver_sdk::price_update::PriceUpdateV2;

#[derive(Accounts)]
pub struct BurnTokens<'info> {
    #[account(mut)]
    pub depositer: Signer<'info>,

    #[account(
        seeds = [SEED_CONFIG_ACCOUNT],
        bump = config_account.bump,
    )]
    pub config_account: Account<'info, Config>,
    pub price_update: Account<'info, PriceUpdateV2>,
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
    #[account(
        mut,
        associated_token::mint = mint_account,
        associated_token::authority = depositer,
        associated_token::token_program = token_program
    )]
    pub token_account: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Program<'info, Token2022>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn process_burn_tokens(ctx: Context<BurnTokens>, amount_to_burn: u64) -> Result<()> {
    let collateral_account = &mut ctx.accounts.collateral_account;
    collateral_account.amount_minted -= amount_to_burn;

    check_health_factor(
        &ctx.accounts.collateral_account,
        &ctx.accounts.config_account,
        &ctx.accounts.price_update,
    )?;

    burn(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Burn {
                mint: ctx.accounts.mint_account.to_account_info(),
                from: ctx.accounts.token_account.to_account_info(),
                authority: ctx.accounts.depositer.to_account_info(),
            },
        ),
        amount_to_burn,
    )?;

    Ok(())
}
