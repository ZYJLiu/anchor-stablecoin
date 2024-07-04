use crate::{
    check_health_factor, deposit_sol_internal, Collateral, Config, SEED_COLLATERAL_ACCOUNT,
    SEED_CONFIG_ACCOUNT, SEED_SOL_ACCOUNT,
};
use anchor_lang::prelude::*;
use pyth_solana_receiver_sdk::price_update::PriceUpdateV2;

#[derive(Accounts)]
pub struct DepositCollateral<'info> {
    #[account(mut)]
    pub depositer: Signer<'info>,

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
    pub system_program: Program<'info, System>,
}

pub fn process_deposit_collateral(
    ctx: Context<DepositCollateral>,
    amount_collateral: u64,
) -> Result<()> {
    deposit_sol_internal(
        &ctx.accounts.depositer,
        &ctx.accounts.sol_account,
        &ctx.accounts.system_program,
        amount_collateral,
    )?;

    let collateral_account = &mut ctx.accounts.collateral_account;
    collateral_account.lamport_balance = ctx.accounts.sol_account.lamports();

    Ok(())
}
