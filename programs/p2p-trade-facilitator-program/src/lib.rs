use anchor_lang::prelude::*;

pub mod state;

declare_id!("FE7D2xK9qJQAYGDDXYTriLxv5f1TL5NEKyw9bVFnuZtt");

#[program]
pub mod p2p_trade_facilitator_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
