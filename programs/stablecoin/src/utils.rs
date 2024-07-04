use crate::{error::CustomError, Collateral, Config, FEED_ID, MAXIMUM_AGE};
use anchor_lang::{prelude::*, solana_program::native_token::LAMPORTS_PER_SOL};
use pyth_solana_receiver_sdk::price_update::{get_feed_id_from_hex, PriceUpdateV2};

pub fn check_health_factor(
    collateral: &Account<Collateral>,
    config: &Account<Config>,
    price_feed: &Account<PriceUpdateV2>,
) -> Result<()> {
    let health_factor = calculate_health_factor(collateral, config, price_feed)?;
    require!(
        health_factor > config.min_health_factor,
        CustomError::BelowMinimumHealthFactor
    );
    Ok(())
}

pub fn calculate_health_factor(
    collateral: &Account<Collateral>,
    config: &Account<Config>,
    price_feed: &Account<PriceUpdateV2>,
) -> Result<u64> {
    if collateral.amount_minted == 0 {
        msg!("Health Factor Max, 0 Tokens Minted");
        return Ok(u64::MAX);
    }

    // Get the collateral value in USD
    // Assuming 1 SOL = $1.00 and $1 = 1_000_000_000
    // Example: get_usd_value(1_000_000_000 lamports, price_feed)
    // collateral_value_in_usd = 1_000_000_000
    let collateral_value_in_usd = get_usd_value(&collateral.lamport_balance, price_feed)?;

    // Adjust the collateral value for the liquidation threshold
    // Example: (1_000_000_000 * 50) / 100 = 500_000_000
    let collateral_adjusted_for_liquidation_threshold =
        (collateral_value_in_usd * config.liquidation_threshold) / 100;

    // Calculate the health factor
    // Example: 500_000_000 / 500_000_000 = 1
    let health_factor = (collateral_adjusted_for_liquidation_threshold) / collateral.amount_minted;

    msg!(
        "Outstanding Token Amount (Minted): {:.9}",
        collateral.amount_minted as f64 / 1e9
    );
    msg!("Health Factor: {}", health_factor);
    Ok(health_factor)
}

fn get_usd_value(amount_in_lamports: &u64, price_feed: &Account<PriceUpdateV2>) -> Result<u64> {
    let feed_id = get_feed_id_from_hex(FEED_ID)?;
    let price = price_feed.get_price_no_older_than(&Clock::get()?, MAXIMUM_AGE, &feed_id)?;

    // Check price is positive
    require!(price.price > 0, CustomError::InvalidPrice);

    // Adjust price to match lamports precision (9 decimals)
    // Example: Assuming 1 SOL = $2.00
    // price.price = 200_000_000 (from Pyth, 8 decimals)
    // price_in_usd = 200_000_000 * 10 = 2_000_000_000 (9 decimals)
    let price_in_usd = price.price as u128 * 10;

    // Calculate USD value
    // Example: Convert 0.5 SOL to USD when 1 SOL = $2.00
    // amount_in_lamports = 500_000_000 (0.5 SOL)
    // price_in_usd = 2_000_000_000 (as calculated above)
    // LAMPORTS_PER_SOL = 1_000_000_000
    // amount_in_usd = (500_000_000 * 2_000_000_000) / 1_000_000_000 = 1_000_000_000 ($1.00)
    let amount_in_usd = (*amount_in_lamports as u128 * price_in_usd) / (LAMPORTS_PER_SOL as u128);

    msg!("*** CONVERT USD TO SOL ***");
    msg!("Price in USD (for 1 SOL): {:.9}", price_in_usd as f64 / 1e9);
    msg!("SOL Amount: {:.9}", *amount_in_lamports as f64 / 1e9);
    msg!("USD Value: {:.9}", amount_in_usd as f64 / 1e9);
    // msg!("Price exponent?: {}", price.exponent);

    Ok(amount_in_usd as u64)
}

pub fn get_lamports_from_usd(
    amount_in_usd: &u64,
    price_feed: &Account<PriceUpdateV2>,
) -> Result<u64> {
    let feed_id = get_feed_id_from_hex(FEED_ID)?;
    let price = price_feed.get_price_no_older_than(&Clock::get()?, MAXIMUM_AGE, &feed_id)?;

    // Check price is positive
    require!(price.price > 0, CustomError::InvalidPrice);

    // Adjust price to match lamports precision (9 decimals)
    // Example: Assuming 1 SOL = $2.00
    // price.price = 200_000_000 (from Pyth, 8 decimals)
    // price_in_usd = 200_000_000 * 10 = 2_000_000_000 (9 decimals)
    let price_in_usd = price.price as u128 * 10;

    // Calculate lamports
    // Example: Convert $0.50 to lamports when 1 SOL = $2.00
    // amount_in_usd = 500_000_000 (user input, 9 decimals for $0.50)
    // LAMPORTS_PER_SOL = 1_000_000_000
    // price_in_usd = 2_000_000_000 (as calculated above)
    // amount_in_lamports = (500_000_000 * 1_000_000_000) / 2_000_000_000 = 250_000_000 (0.25 SOL)
    let amount_in_lamports = ((*amount_in_usd as u128) * (LAMPORTS_PER_SOL as u128)) / price_in_usd;

    msg!("*** CONVERT SOL TO USD ***");
    msg!("Price in USD (for 1 SOL): {:.9}", price_in_usd as f64 / 1e9);
    msg!("USD Amount: {:.9}", *amount_in_usd as f64 / 1e9);
    msg!("SOL Value: {:.9}", amount_in_lamports as f64 / 1e9);

    Ok(amount_in_lamports as u64)
}