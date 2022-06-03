use crate::{error, state, BuyTicketWithSolAccount, BuyTicketWithSplAccount};
use anchor_lang::prelude::*;
use anchor_spl::token;

impl<'info> BuyTicketWithSolAccount<'info> {
    pub fn process(
        &mut self,
        amount: u8,
    ) -> Result<()> {
        if self.lottery.status != state::LotteryStatus::Opened {
            return Err(error::ErrorCode::LotteryNotOpen.into());
        }

        let ticket = &mut self.ticket;
        let lottery = &mut self.lottery;

        // check end date
        if self.clock_sysvar.unix_timestamp >= lottery.end_date {
            return Err(error::ErrorCode::ExpireDateInThePast.into());
        }
        
        if ticket.tickets.checked_add(amount as u64).unwrap() > lottery.limit_tickets {
            return Err(error::ErrorCode::TicketLimited.into());
        }
        if lottery.remain_tickets.checked_sub(amount as u64).unwrap() < 0 as u64 {
            return Err(error::ErrorCode::AllTicketSold.into());
        }

        let ix = anchor_lang::solana_program::system_instruction::transfer(
                                    &self.buyer.key(), 
                                    &self.vault.key(), 
                                    lottery.ticket_price.checked_mul(amount as u64).unwrap());
        anchor_lang::solana_program::program::invoke(&ix, &[
                                                                self.buyer.to_account_info(), 
                                                                self.vault.to_account_info(), 
                                                            ])?;

        ticket.tickets = ticket.tickets.checked_add(amount as u64).unwrap();

        lottery.remain_tickets = lottery.remain_tickets.checked_sub(amount as u64).unwrap();
        
        Ok(())
    }
}

impl<'info> BuyTicketWithSplAccount<'info> {
    pub fn process(
        &mut self,
        amount: u8,
    ) -> Result<()> {
        if self.lottery.status != state::LotteryStatus::Opened {
            return Err(error::ErrorCode::LotteryNotOpen.into());
        }

        let ticket = &mut self.ticket;
        let lottery = &mut self.lottery;
        
        // check end date
        if self.clock_sysvar.unix_timestamp >= lottery.end_date {
            return Err(error::ErrorCode::ExpireDateInThePast.into());
        }

        if ticket.tickets.checked_add(amount as u64).unwrap() > lottery.limit_tickets {
            return Err(error::ErrorCode::TicketLimited.into());
        }
        if lottery.remain_tickets.checked_sub(amount as u64).unwrap() < 0 as u64 {
            return Err(error::ErrorCode::AllTicketSold.into());
        }


        let cpi_ctx = CpiContext::new(
            self.token_program.to_account_info(),
            token::Transfer {
                from: self.buyer_token_account.to_account_info(),
                to: self.vault.to_account_info(),
                authority: self.buyer.to_account_info(), //todo use user account as signer
            },
        );
        token::transfer(cpi_ctx, lottery.ticket_price.checked_mul(amount as u64).unwrap())?;

        ticket.tickets = ticket.tickets.checked_add(amount as u64).unwrap();
        
        lottery.remain_tickets = lottery.remain_tickets.checked_sub(amount as u64).unwrap();
        
        Ok(())
    }
}