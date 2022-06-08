use anchor_lang::{prelude::*};

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub enum LotteryStatus {
    Opened,
    Closed,
    Completed,
}

#[account]
#[derive(Debug)]
pub struct Lottery {
    /// current lottery status
    pub status: LotteryStatus,

    /// creator of the lottery
    pub creator: Pubkey,

    /// mint spl
    pub mint: Option<Pubkey>,

    /// vault that holds the sol or spl of the ticket price
    pub vault: Pubkey,

    /// open date
    pub start_date: i64,

    /// close date
    pub end_date: i64,

    /// ticket price
    pub ticket_price: u64,

    /// ticket numbers
    pub ticket_numbers: u64,

    /// remain tickets
    pub remain_tickets: u64,

    /// limit tickets
    pub limit_tickets: u64,

    /// winners
    pub winners: [Pubkey; 10],

    /// winner numbers
    pub winner_numbers: u64,

    /// bump
    pub bump: u8,

    /// key
    pub lottery_key: Pubkey,
}

impl Lottery {
    pub const LEN: usize = 8 + 1 + 32 + 33 + 32 + 8 + 8 + 8 + 8 + 8 + 8 + 32*10 + 8 + 1 + 32;
}