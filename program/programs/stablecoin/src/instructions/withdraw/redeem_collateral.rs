use crate::{
    check_health_factor, withdraw_sol_internal, Collateral, Config, SEED_COLLATERAL_ACCOUNT,
    SEED_CONFIG_ACCOUNT,
};
use anchor_lang::prelude::*;
use pyth_solana_receiver_sdk::price_update::PriceUpdateV2;

#[derive(Accounts)]
pub struct RedeemCollateral<'info> {
    #[account(mut)]
    pub depositor: Signer<'info>,

    #[account(
        seeds = [SEED_CONFIG_ACCOUNT],
        bump = config_account.bump,
    )]
    pub config_account: Account<'info, Config>,
    #[account(
        mut,
        seeds = [SEED_COLLATERAL_ACCOUNT, depositor.key().as_ref()],
        bump = collateral_account.bump,
        has_one = depositor,
        has_one = sol_account
    )]
    pub collateral_account: Account<'info, Collateral>,
    #[account(mut)]
    pub sol_account: SystemAccount<'info>,
    pub price_update: Account<'info, PriceUpdateV2>,
    pub system_program: Program<'info, System>,
}

// https://github.com/Cyfrin/foundry-defi-stablecoin-cu/blob/main/src/DSCEngine.sol#L177
pub fn process_redeem_collateral(
    ctx: Context<RedeemCollateral>,
    amount_collateral: u64,
) -> Result<()> {
    let collateral_account = &mut ctx.accounts.collateral_account;
    collateral_account.lamport_balance = ctx.accounts.sol_account.lamports() - amount_collateral;

    check_health_factor(
        &ctx.accounts.collateral_account,
        &ctx.accounts.config_account,
        &ctx.accounts.price_update,
    )?;

    withdraw_sol_internal(
        &ctx.accounts.sol_account,
        &ctx.accounts.depositor.to_account_info(),
        &ctx.accounts.system_program,
        &ctx.accounts.depositor.key(),
        ctx.accounts.collateral_account.bump_sol_account,
        amount_collateral,
    )?;
    Ok(())
}
