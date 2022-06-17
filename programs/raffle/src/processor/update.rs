use crate:: {error, UpdateLotteryAccount, UpdateDiscountAccount};
use anchor_lang::{prelude::*, solana_program::clock::UnixTimestamp};

impl<'info> UpdateLotteryAccount<'info> {
    pub fn process(
        &mut self,
        start_date: UnixTimestamp,
        end_date: UnixTimestamp,
        ticket_price: u64,
        ticket_numbers: u64,
        limit_tickets: u64,
        winners: u64,
    ) -> Result<()> {
        if winners > 10 {
            return Err(error::ErrorCode::MaxWinnerError.into());
        }

        // check end date
        if self.clock_sysvar.unix_timestamp >= self.lottery.end_date {
            return Err(error::ErrorCode::ExpireDateInThePast.into());
        }

        // check the start date
        if self.lottery.start_date >= self.lottery.end_date {
            return Err(error::ErrorCode::ExpireDateInThePast.into());
        }

        // check start date
        if self.clock_sysvar.unix_timestamp >= self.lottery.start_date {
            return Err(error::ErrorCode::ExpireDateInThePast.into());
        }

        self.lottery.start_date = start_date;
        self.lottery.end_date = end_date;
        self.lottery.ticket_price = ticket_price;
        self.lottery.ticket_numbers = ticket_numbers;
        self.lottery.remain_tickets = ticket_numbers;
        self.lottery.limit_tickets = limit_tickets;
        self.lottery.winner_numbers = winners;

        Ok(())
    }
}

impl<'info> UpdateDiscountAccount<'info> {
    pub fn process(
        &mut self,
        discount_value: u8
    ) -> Result<()> {

        // check end date
        if self.clock_sysvar.unix_timestamp >= self.lottery.start_date {
            return Err(error::ErrorCode::ExpireDateInThePast.into());
        }

        let discount = &mut self.discount;
        discount.discount = discount_value;
        
        Ok(())
    }
}