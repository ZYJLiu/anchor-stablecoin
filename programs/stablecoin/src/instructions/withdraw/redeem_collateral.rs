use crate::{
    check_health_factor, withdraw_sol_internal, Collateral, Config, SEED_COLLATERAL_ACCOUNT,
    SEED_CONFIG_ACCOUNT, SEED_SOL_ACCOUNT,
};
use anchor_lang::prelude::*;
use pyth_solana_receiver_sdk::price_update::PriceUpdateV2;

#[derive(Accounts)]
pub struct RedeemCollateral<'info> {
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
        seeds = [SEED_SOL_ACCOUNT, depositer.key().as_ref()],
        bump = collateral_account.bump_sol_account,
    )]
    pub sol_account: SystemAccount<'info>,
    pub price_update: Account<'info, PriceUpdateV2>,
    pub system_program: Program<'info, System>,
}

pub fn process_redeem_collateral(
    ctx: Context<RedeemCollateral>,
    amount_collateral: u64,
) -> Result<()> {
    let collateral_account = &mut ctx.accounts.collateral_account;
    collateral_account.lamport_balance -= amount_collateral;

    check_health_factor(
        &ctx.accounts.collateral_account,
        &ctx.accounts.config_account,
        &ctx.accounts.price_update,
    )?;

    withdraw_sol_internal(
        &ctx.accounts.sol_account,
        &ctx.accounts.depositer.to_account_info(),
        &ctx.accounts.system_program,
        &ctx.accounts.depositer.key(),
        ctx.accounts.collateral_account.bump_sol_account,
        amount_collateral,
    )?;
    Ok(())
}
