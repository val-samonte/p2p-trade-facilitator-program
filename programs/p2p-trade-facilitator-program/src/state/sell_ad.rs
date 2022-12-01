use anchor_lang::prelude::*;

#[account]
pub struct SellAd {
    /// Bump nonce of the PDA (1).
    pub bump: u8,

    /// The id of the ad (32)
    pub id: Pubkey,

    /// The owner of the ad (32)
    pub owner: Pubkey,

    /// The id of the device on which this ad is created (32)
    pub device: Pubkey,

    /// Price in peso per token (8)
    pub unit_price: u64,

    /// Amount assigned to this posted ad in lamports (8)
    pub available: u64,

    /// Minimum purchase in peso (8)
    pub min_limit: u64,

    /// Maximum purchase in peso (8)
    pub max_limit: u64,

    /// Binary flags indicating the transfer methods available for this post (4)
    /// 0000000 All payments
    /// 0000001 Gcash
    /// 0000002 Bank Transfer
    /// 0000004 UnionBank of the Philippines
    /// 0000008 Paymaya
    /// 0000016 Bank of the Philippine Islands...
    /// 0000032 SEA Bank
    /// 0000064 Banco De Oro (BDO)
    /// 0000128 Coins.ph
    /// 0000256 Metropolitan Bank of the Phili...
    /// 0000512 Rizal Commercial Banking Corpo...
    /// 0001024 Landbank of the Philippines
    /// 0002048 Philippines National Bank (PNB...
    /// 0004096 CIMB Philippines
    /// 0008192 ShopeePay-SEA
    /// 0016384 Asia United Bank
    /// 0032768 Maybank
    /// 0065536 Alipay
    /// 0131072 Cash Deposit to Bank
    /// 0262144 Sterling Bank
    /// 0524288 7-Eleven
    /// 1048576 CIMB Niaga
    /// 2097152 LINE Pay
    /// 4194304 WeChat
    pub transfer_method: u32,

    /// State of the ad (1)
    /// 0 open
    /// 1 buyer to proceed with transferring funds to seller
    /// 2 seller to confirm transferred funds and close the deal
    /// 3 appeal / call for public hearing
    pub state: u8,

    /// Assigned buyer, if present, funds will be locked until trade is settled (1 + 32)
    pub buyer: Option<Pubkey>,

    /// Time when the buyer and the seller agreed to proceed with the trade (1 + 8)
    pub time_started: Option<u64>,

    /// Unused reserved byte space for additive future changes (128)
    pub _reserved: [u8; 128],
}

impl SellAd {
    pub const LEN: usize = 1 + 32 + 32 + 32 + 8 + 8 + 8 + 8 + 4 + 1 + (1 + 32) + (1 + 8) + 128;
}
