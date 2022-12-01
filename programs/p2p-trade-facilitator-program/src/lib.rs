use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;

pub use instructions::*;

declare_id!("FE7D2xK9qJQAYGDDXYTriLxv5f1TL5NEKyw9bVFnuZtt");

#[program]
pub mod p2p_trade_facilitator_program {
    use super::*;

    pub fn create_sell_ad(ctx: Context<CreateSellAd>, params: CreateSellAdParams) -> Result<()> {
        instructions::create_sell_ad_handler(ctx, params)
    }
}

#[error_code]
pub enum CustomError {
    #[msg("The amount available is not within the limit range")]
    AmountNotWithinLimit,
}
