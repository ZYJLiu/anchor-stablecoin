use crate::{deposit_sol_internal, Collateral, SEED_COLLATERAL_ACCOUNT};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct DepositCollateral<'info> {
    #[account(mut)]
    pub depositor: Signer<'info>,

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
    pub system_program: Program<'info, System>,
}

// https://github.com/Cyfrin/foundry-defi-stablecoin-cu/blob/main/src/DSCEngine.sol#L269
pub fn process_deposit_collateral(
    ctx: Context<DepositCollateral>,
    amount_collateral: u64,
) -> Result<()> {
    deposit_sol_internal(
        &ctx.accounts.depositor,
        &ctx.accounts.sol_account,
        &ctx.accounts.system_program,
        amount_collateral,
    )?;

    let collateral_account = &mut ctx.accounts.collateral_account;
    collateral_account.lamport_balance = ctx.accounts.sol_account.lamports();

    Ok(())
}
