use crate::{error, state, utils, CloseLottery, CloseAccount};
use anchor_lang::prelude::*;

impl<'info> CloseLottery<'info> {
    pub fn process(
        &mut self,
        winners: [Pubkey; 10],
    ) -> Result<()> {
        if self.lottery.status != state::LotteryStatus::Opened {
            return Err(error::ErrorCode::LotteryNotOpen.into());
        }
        
        self.lottery.status = state::LotteryStatus::Closed;
        self.lottery.winners = winners;
        
        Ok(())
    }
}

impl<'info> CloseAccount<'info> {
    pub fn process(
        &mut self,
    ) -> Result<()> {
        if self.lottery.status != state::LotteryStatus::Closed {
            return Err(error::ErrorCode::LotteryOpened.into());
        }
        
        utils::delete_account(&self.ticket, &self.receiver)
    }
}