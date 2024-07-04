use crate::{Config, SEED_CONFIG_ACCOUNT};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct UpdateConfig<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [SEED_CONFIG_ACCOUNT],
        bump,
        has_one = authority
    )]
    pub config_account: Account<'info, Config>,
}

// change health factor to test liquidate instruction
pub fn process_update_config(ctx: Context<UpdateConfig>) -> Result<()> {
    let config_account = &mut ctx.accounts.config_account;
    config_account.min_health_factor = 100;

    msg!("{:#?}", config_account);
    Ok(())
}
