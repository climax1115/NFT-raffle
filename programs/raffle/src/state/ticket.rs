use anchor_lang::{prelude::*};

#[account]
#[derive(Debug)]
pub struct Ticket {
    /// owner of the ticket
    pub owner: Pubkey,

    /// pubkey of the lottery
    pub lottery: Pubkey,

    /// tickets
    pub tickets: u64,

    pub bump: u8,
}

impl Ticket {
    pub const LEN: usize = 8 + 32 + 32 + 8 + 1;
}