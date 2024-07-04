use crate::{
    burn_tokens_internal, Collateral, Config, SEED_COLLATERAL_ACCOUNT, SEED_CONFIG_ACCOUNT,
};
use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, Token2022, TokenAccount};

#[derive(Accounts)]
pub struct BurnTokens<'info> {
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
        has_one = token_account,
    )]
    pub collateral_account: Account<'info, Collateral>,
    #[account(mut)]
    pub mint_account: InterfaceAccount<'info, Mint>,
    #[account(mut)]
    pub token_account: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Program<'info, Token2022>,
}

// https://github.com/Cyfrin/foundry-defi-stablecoin-cu/blob/main/src/DSCEngine.sol#L195
pub fn process_burn_tokens(ctx: Context<BurnTokens>, amount_to_burn: u64) -> Result<()> {
    let collateral_account = &mut ctx.accounts.collateral_account;
    collateral_account.amount_minted -= amount_to_burn;

    burn_tokens_internal(
        &ctx.accounts.mint_account,
        &ctx.accounts.token_account,
        &ctx.accounts.depositor,
        &ctx.accounts.token_program,
        amount_to_burn,
    )?;
    Ok(())
}
