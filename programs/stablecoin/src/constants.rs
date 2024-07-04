use anchor_lang::prelude::*;

pub const SEED_CONFIG_ACCOUNT: &[u8] = b"config";
pub const SEED_COLLATERAL_ACCOUNT: &[u8] = b"collateral";
pub const SEED_SOL_ACCOUNT: &[u8] = b"sol";
pub const SEED_MINT_ACCOUNT: &[u8] = b"mint";

#[constant]
pub const FEED_ID: &str = "0xef0d8b6fda2ceba41da15d4095d1da392a0d2f8ed0c6c7bc0f4cfac8c280b56d";
pub const MAXIMUM_AGE: u64 = 50;
