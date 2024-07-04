use crate::error::CustomError;
use crate::{
    calculate_health_factor, get_lamports_from_usd, Collateral, Config, SEED_CONFIG_ACCOUNT,
    SEED_MINT_ACCOUNT, SEED_SOL_ACCOUNT,
};
use anchor_lang::prelude::*;
use anchor_lang::solana_program::native_token::LAMPORTS_PER_SOL;
use anchor_lang::system_program::{transfer, Transfer};
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_2022::{burn, Burn};
use anchor_spl::token_interface::{Mint, Token2022, TokenAccount};
use pyth_solana_receiver_sdk::price_update::PriceUpdateV2;

#[derive(Accounts)]
pub struct Liquidate<'info> {
    #[account(mut)]
    pub liquidator: Signer<'info>,

    pub price_update: Account<'info, PriceUpdateV2>,
    #[account(
        seeds = [SEED_CONFIG_ACCOUNT],
        bump = config_account.bump,
    )]
    pub config_account: Account<'info, Config>,
    #[account(
        mut,
        has_one = sol_account
        // seeds = [SEED_COLLATERAL_ACCOUNT, depositer.key().as_ref()],
        // bump = collateral_account.bump,
    )]
    pub collateral_account: Account<'info, Collateral>,
    #[account(
        mut,
        // seeds = [SEED_SOL_ACCOUNT, depositer.key().as_ref()],
        // bump,
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
        associated_token::authority = liquidator,
        associated_token::token_program = token_program
    )]
    pub token_account: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Program<'info, Token2022>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn process_liquidate(ctx: Context<Liquidate>, amount_to_burn: u64) -> Result<()> {
    let health_factor = calculate_health_factor(
        &ctx.accounts.collateral_account,
        &ctx.accounts.config_account,
        &ctx.accounts.price_update,
    )?;

    require!(
        health_factor < ctx.accounts.config_account.min_health_factor,
        CustomError::AboveMinimumHealthFactor
    );

    let lamports = get_lamports_from_usd(&amount_to_burn, &ctx.accounts.price_update)?;
    let liquidation_bonus =
        lamports * ctx.accounts.config_account.liquidation_bonus / LAMPORTS_PER_SOL;
    let amount_liquidated = lamports + liquidation_bonus;

    let depositer_key = ctx.accounts.collateral_account.depositor;
    let bump_sol_account = ctx.accounts.collateral_account.bump_sol_account;
    let signer_seeds: &[&[&[u8]]] = &[&[
        SEED_SOL_ACCOUNT,
        depositer_key.as_ref(),
        &[bump_sol_account],
    ]];
    transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            Transfer {
                from: ctx.accounts.sol_account.to_account_info(),
                to: ctx.accounts.liquidator.to_account_info(),
            },
        )
        .with_signer(signer_seeds),
        amount_liquidated,
    )?;

    burn(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Burn {
                mint: ctx.accounts.mint_account.to_account_info(),
                from: ctx.accounts.token_account.to_account_info(),
                authority: ctx.accounts.liquidator.to_account_info(),
            },
        ),
        amount_to_burn,
    )?;

    let collateral_account = &mut ctx.accounts.collateral_account;
    collateral_account.lamport_balance -= amount_liquidated;
    collateral_account.amount_minted -= amount_to_burn;

    // msg!("{:#?}", collateral_account);

    calculate_health_factor(
        &ctx.accounts.collateral_account,
        &ctx.accounts.config_account,
        &ctx.accounts.price_update,
    )?;

    Ok(())
}
