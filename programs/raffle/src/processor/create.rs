use crate:: {error, state, CreateSolLotteryAccount, CreateSplLotteryAccount, CreateTicketAccount};
use anchor_lang::{prelude::*, solana_program::clock::UnixTimestamp};


impl<'info> CreateSolLotteryAccount<'info> {
    pub fn process(
        &mut self,
        lottery_bump: u8,
        lottery_key: Pubkey,
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
        self.lottery.status = state::LotteryStatus::Opened;

        self.lottery.creator = self.creator.key().clone();

        self.lottery.vault = self.vault.key().clone();

        self.lottery.start_date = start_date;
        self.lottery.end_date = end_date;
        self.lottery.ticket_price = ticket_price;
        self.lottery.ticket_numbers = ticket_numbers;
        self.lottery.remain_tickets = ticket_numbers;
        self.lottery.limit_tickets = limit_tickets;
        self.lottery.winner_numbers = winners;
        self.lottery.bump = lottery_bump;
        self.lottery.lottery_key = lottery_key;

        // check end date
        if self.clock_sysvar.unix_timestamp >= self.lottery.end_date {
            return Err(error::ErrorCode::ExpireDateInThePast.into());
        }

        // check the start date
        if self.lottery.start_date >= self.lottery.end_date {
            return Err(error::ErrorCode::ExpireDateInThePast.into());
        }

        Ok(())
    }
}

impl<'info> CreateSplLotteryAccount<'info> {
    pub fn process(
        &mut self,
        lottery_bump: u8,
        lottery_key: Pubkey,
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
        self.lottery.status = state::LotteryStatus::Opened;

        self.lottery.mint = Some(self.mint.key().clone());

        self.lottery.creator = self.creator.key().clone();

        self.lottery.vault = self.vault.key().clone();

        self.lottery.start_date = start_date;
        self.lottery.end_date = end_date;
        self.lottery.ticket_price = ticket_price;
        self.lottery.ticket_numbers = ticket_numbers;
        self.lottery.remain_tickets = ticket_numbers;
        self.lottery.limit_tickets = limit_tickets;
        self.lottery.winner_numbers = winners;
        self.lottery.lottery_key = lottery_key;
        self.lottery.bump = lottery_bump;

        // check end date
        if self.clock_sysvar.unix_timestamp >= self.lottery.end_date {
            return Err(error::ErrorCode::ExpireDateInThePast.into());
        }

        // check the start date
        if self.lottery.start_date >= self.lottery.end_date {
            return Err(error::ErrorCode::ExpireDateInThePast.into());
        }

        Ok(())
    }
}

impl<'info> CreateTicketAccount<'info> {
	pub fn process(&mut self, bump: u8) -> Result<()> {

        // check end date
        if self.clock_sysvar.unix_timestamp >= self.lottery.end_date {
            return Err(error::ErrorCode::ExpireDateInThePast.into());
        }

		let ticket = &mut self.ticket;
		ticket.owner = self.buyer.key().clone();
		ticket.lottery = self.lottery.key().clone();
		ticket.tickets = 0;
		ticket.bump = bump;
        Ok(())
	}
}