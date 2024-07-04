use crate::{
    check_health_factor, Collateral, Config, SEED_COLLATERAL_ACCOUNT, SEED_CONFIG_ACCOUNT,
    SEED_MINT_ACCOUNT, SEED_SOL_ACCOUNT,
};
use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_2022::{burn, Burn};
use anchor_spl::token_interface::{Mint, Token2022, TokenAccount};
use pyth_solana_receiver_sdk::price_update::PriceUpdateV2;

#[derive(Accounts)]
pub struct RedeemCollateralAndBurnTokens<'info> {
    #[account(mut)]
    pub depositer: Signer<'info>,

    pub price_update: Account<'info, PriceUpdateV2>,
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
        seeds = [SEED_SOL_ACCOUNT, depositer.key().as_ref()],
        bump = collateral_account.bump_sol_account,
    )]
    pub sol_account: SystemAccount<'info>,
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

pub fn process_redeem_collateral_and_burn_tokens(
    ctx: Context<RedeemCollateralAndBurnTokens>,
    amount_collateral: u64,
    amount_to_burn: u64,
) -> Result<()> {
    let collateral_account = &mut ctx.accounts.collateral_account;
    collateral_account.lamport_balance -= amount_collateral;
    collateral_account.amount_minted -= amount_to_burn;

    check_health_factor(
        &ctx.accounts.collateral_account,
        &ctx.accounts.config_account,
        &ctx.accounts.price_update,
    )?;

    let depositer_key = ctx.accounts.depositer.key();
    let bump = ctx.accounts.collateral_account.bump_sol_account;
    let signer_seeds: &[&[&[u8]]] = &[&[SEED_SOL_ACCOUNT, depositer_key.as_ref(), &[bump]]];

    transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            Transfer {
                from: ctx.accounts.depositer.to_account_info(),
                to: ctx.accounts.sol_account.to_account_info(),
            },
        )
        .with_signer(signer_seeds),
        amount_collateral,
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
