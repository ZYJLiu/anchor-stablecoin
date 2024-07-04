use anchor_lang::prelude::*;
use anchor_spl::token_interface::{
   Mint, Token2022,
};
use crate::{Config, SEED_CONFIG_ACCOUNT, SEED_MINT_ACCOUNT};

#[derive(Accounts)]
pub struct InitializeConfig<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init, 
        payer = authority, 
        space = 8 + Config::INIT_SPACE,
        seeds = [SEED_CONFIG_ACCOUNT],
        bump,
    )]
    pub config_account: Account<'info, Config>,
    #[account(
        init,
        payer = authority,
        seeds = [SEED_MINT_ACCOUNT],
        bump,
        mint::decimals = 9,
        mint::authority = mint_account,
        mint::freeze_authority = mint_account,
        mint::token_program = token_program
    )]
    pub mint_account: InterfaceAccount<'info, Mint>,
    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
}

pub fn process_initialize_config(ctx: Context<InitializeConfig>) -> Result<()> {
    let config_account = &mut ctx.accounts.config_account;
    config_account.authority = ctx.accounts.authority.key();
    config_account.liquidation_threshold = 50; // This means you need to be 200% over-collateralized;
    config_account.liquidation_bonus = 10; // This means you get assets at a 10% discount when liquidating
    config_account.min_health_factor = 1;
    config_account.bump = ctx.bumps.config_account;
    config_account.bump_mint_account = ctx.bumps.mint_account;
    Ok(())
}

