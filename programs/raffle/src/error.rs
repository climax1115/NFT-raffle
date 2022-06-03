use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    /// 6000.
    #[msg("The end date is in the past.")]
    ExpireDateInThePast,

    /// 6001.
    #[msg("The start date is in the past.")]
    StartDateInThePast,

    /// 6002.
    #[msg("The Lottery is now closed.")]
    InvalidLotteryStatus,

    /// 6003.
    #[msg("Lottery is closed.")]
    LotteryIsClosed,

    /// 6004.
    #[msg("The Lottery is not started.")]
    LotteryIsNotStarted,

    /// 6005.
    #[msg("The Lottery is not opened.")]
    LotteryNotOpen,

    /// 6006.
    #[msg("The ticket is not valid.")]
    InvalidTicket,

    /// 6007.
    #[msg("There is no ticket in the account.")]
    NoTicket,

    /// 6008.
    #[msg("You are not the owner of the ticket.")]
    NotOwner,

    /// 6009.
    #[msg("The ticket has exceeded the limit amount.")]
    TicketLimited,

    /// 6010.
    #[msg("There is not available tickets.")]
    AllTicketSold,

    /// 6011.
    #[msg("Max winner number is 10.")]
    MaxWinnerError,

    /// 6012.
    #[msg("The Lottery is opened.")]
    LotteryOpened,
}
