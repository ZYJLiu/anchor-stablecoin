use anchor_lang::prelude::*;
use instructions::*;
mod instructions;
use state::*;
mod state;
use utils::*;
pub mod constants;
mod error;
mod utils;
pub use constants::*;

declare_id!("6DjiD8tQhJ9ZS3WZrwNubfoBRBrqfWacNR3bXBQ7ir91");

#[program]
pub mod stablecoin {
    use super::*;

    pub fn initialize_config(ctx: Context<InitializeConfig>) -> Result<()> {
        process_initialize_config(ctx)
    }

    pub fn update_config(ctx: Context<UpdateConfig>) -> Result<()> {
        process_update_config(ctx)
    }

    pub fn deposit_collateral_and_mint(
        ctx: Context<DepositCollateralAndMintTokens>,
        amount_collateral: u64,
        amount_to_mint: u64,
    ) -> Result<()> {
        process_deposit_collateral_and_mint_tokens(ctx, amount_collateral, amount_to_mint)
    }

    pub fn deposit_collateral(
        ctx: Context<DepositCollateral>,
        amount_collateral: u64,
    ) -> Result<()> {
        process_deposit_collateral(ctx, amount_collateral)
    }

    pub fn mint_tokens(ctx: Context<MintTokens>, amount_to_mint: u64) -> Result<()> {
        process_mint_tokens(ctx, amount_to_mint)
    }

    pub fn redeem_collateral(ctx: Context<RedeemCollateral>, amount_collateral: u64) -> Result<()> {
        process_redeem_collateral(ctx, amount_collateral)
    }

    pub fn burn_tokens(ctx: Context<BurnTokens>, amount_to_burn: u64) -> Result<()> {
        process_burn_tokens(ctx, amount_to_burn)
    }

    pub fn redeem_collateral_and_burn_tokens(
        ctx: Context<RedeemCollateralAndBurnTokens>,
        amount_collateral: u64,
        amount_to_burn: u64,
    ) -> Result<()> {
        process_redeem_collateral_and_burn_tokens(ctx, amount_collateral, amount_to_burn)
    }

    pub fn liquidate(ctx: Context<Liquidate>, amount_to_burn: u64) -> Result<()> {
        process_liquidate(ctx, amount_to_burn)
    }
}
