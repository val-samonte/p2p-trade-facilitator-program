use anchor_lang::{prelude::*, solana_program::native_token::LAMPORTS_PER_SOL, system_program};

use crate::{state::SellAd, CustomError};

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct CreateSellAdParams {
    pub unit_price: u64,
    pub available: u64,
    pub min_limit: u64,
    pub max_limit: u64,
    pub transfer_method: u32,
}

#[derive(Accounts)]
#[instruction(params: CreateSellAdParams)]
pub struct CreateSellAd<'info> {
    #[account(
        init,
        payer = authority,
        seeds = [
            "sell_ad".as_bytes(),
            id.key().as_ref(),
        ],
        bump,
        space = SellAd::LEN,
    )]
    pub sell_ad: Account<'info, SellAd>,

    pub id: Signer<'info>,

    pub device: Signer<'info>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn create_sell_ad_handler(
    ctx: Context<CreateSellAd>,
    params: CreateSellAdParams,
) -> Result<()> {
    let sell_ad_bump = *ctx.bumps.get("sell_ad").unwrap();
    let sell_ad = &mut ctx.accounts.sell_ad;
    let authority = &mut ctx.accounts.authority;
    let id = &ctx.accounts.id;
    let device = &ctx.accounts.device;

    sell_ad.bump = sell_ad_bump;
    sell_ad.id = id.key();
    sell_ad.authority = authority.key();
    sell_ad.device = device.key();

    let available_in_peso = params
        .available
        .checked_mul(params.unit_price)
        .unwrap()
        .checked_div(LAMPORTS_PER_SOL / 100)
        .unwrap();

    if available_in_peso > params.max_limit || available_in_peso < params.min_limit {
        return Err(error!(CustomError::AmountNotWithinLimit));
    }

    sell_ad.unit_price = params.unit_price;
    sell_ad.available = params.available;
    sell_ad.min_limit = params.min_limit;
    sell_ad.max_limit = params.max_limit;
    sell_ad.transfer_method = params.transfer_method;
    sell_ad.state = 0;

    // deduct SOL funds from owner and transfer it to the SellAd account

    let cpi_ctx = CpiContext::new(
        ctx.accounts.system_program.to_account_info(),
        system_program::Transfer {
            from: ctx.accounts.authority.to_account_info(),
            to: ctx.accounts.sell_ad.to_account_info(),
        },
    );

    system_program::transfer(cpi_ctx, params.available)?;

    Ok(())
}
