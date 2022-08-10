pub mod state;
pub mod utils;
mod processor;
pub mod error;

use anchor_lang::prelude::*;
use anchor_spl::token::{ Token, TokenAccount, Mint };

declare_id!("BLV8oNcV6cZVvUJoQjK7V1gLEdPdj5u9AKg4StGBtALM");

#[program]
pub mod raffle {
    use super::*;

    pub fn create_sol_lottery(
            ctx: Context<CreateSolLotteryAccount>, 
            lottery_bump: u8,
            lottery_key: Pubkey,
            start_date: i64,
            end_date: i64,
            ticket_price: u64,
            ticket_numbers: u64,
            limit_tickets: u64,
            winners: u64
        ) -> Result<()> {
        ctx.accounts.process(
            lottery_bump,
            lottery_key,
            start_date,
            end_date,
            ticket_price,
            ticket_numbers,
            limit_tickets,
            winners
        )
    }

    pub fn create_spl_lottery(
            ctx: Context<CreateSplLotteryAccount>,
            lottery_bump: u8,
            lottery_key: Pubkey,
            start_date: i64,
            end_date: i64,
            ticket_price: u64,
            ticket_numbers: u64,
            limit_tickets: u64,
            winners: u64
        ) -> Result<()> {
        ctx.accounts.process(
            lottery_bump,
            lottery_key,
            start_date,
            end_date,
            ticket_price,
            ticket_numbers,
            limit_tickets,
            winners
        )
    }

    pub fn update_lottery(
            ctx: Context<UpdateLotteryAccount>, 
            start_date: i64,
            end_date: i64,
            ticket_price: u64,
            ticket_numbers: u64,
            limit_tickets: u64,
            winners: u64
        ) -> Result<()> {
        ctx.accounts.process(
            start_date,
            end_date,
            ticket_price,
            ticket_numbers,
            limit_tickets,
            winners
        )
    }

    pub fn create_ticket(ctx: Context<CreateTicketAccount>, bump: u8) -> Result<()> {
        ctx.accounts.process(
            bump
        )
    }

    pub fn buy_ticket_with_sol<'key, 'accounts, 'remaining, 'info>(
        ctx: Context<'key, 'accounts, 'remaining, 'info, BuyTicketWithSolAccount<'info>>,
        amount: u8,
    ) -> Result<()> {
        let rem_acc = &ctx.remaining_accounts;
        ctx.accounts.process(amount, rem_acc.to_vec())
    }

    pub fn buy_ticket_with_spl<'key, 'accounts, 'remaining, 'info>(
        ctx: Context<'key, 'accounts, 'remaining, 'info, BuyTicketWithSplAccount<'info>>,
        amount: u8,
    ) -> Result<()> {
        let rem_acc = &ctx.remaining_accounts;
        ctx.accounts.process(amount, rem_acc.to_vec())
    }

    pub fn close_lottery(
        ctx: Context<CloseLottery>,
        winners: [Pubkey; 10],
    ) -> Result<()> {
        ctx.accounts.process(winners)
    }

    pub fn close_account(
        ctx: Context<CloseAccount>,
    ) -> Result<()> {
        ctx.accounts.process()
    }

    pub fn create_discount(
        ctx: Context<CreateDiscountAccount>,
        bump: u8,
        discount_type: u8,
        verifier: Pubkey,
        discount: u8,
    ) -> Result<()> {
        ctx.accounts.process(bump, discount_type, verifier, discount)
    }

    pub fn update_discount(
        ctx: Context<UpdateDiscountAccount>,
        discount: u8,
    ) -> Result<()> {
        ctx.accounts.process(discount)
    }
}

#[derive(Accounts)]
#[instruction(lottery_bump: u8, lottery_key: Pubkey)]
pub struct CreateSolLotteryAccount<'info> {
    /// lottery info
    #[account(
        init, 
        payer = creator, 
        space = state::Lottery::LEN,
        seeds = [
            utils::LOTTERY_PREFIX.as_bytes(), 
            creator.key().as_ref(), 
            lottery_key.key().as_ref(),
        ],
        bump,
    )]
    pub lottery: Box<Account<'info, state::Lottery>>,

    /// the creator of the lottery
    #[account(mut)]
    pub creator: Signer<'info>,

    /// will hold the sol of the raffles tickets price
    /// CHECK: it's alright
    pub vault: AccountInfo<'info>,

    pub rent_sysvar: Sysvar<'info, Rent>,
    pub clock_sysvar: Sysvar<'info, Clock>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(lottery_bump: u8, lottery_key: Pubkey)]
pub struct CreateSplLotteryAccount<'info> {
    /// lottery info
    #[account(
        init, 
        payer = creator, 
        space = state::Lottery::LEN,
        seeds = [
            utils::LOTTERY_PREFIX.as_bytes(), 
            creator.key().as_ref(), 
            lottery_key.key().as_ref(),
        ],
        bump,
    )]
    pub lottery: Box<Account<'info, state::Lottery>>,

    /// the creator of the lottery
    #[account(mut)]
    pub creator: Signer<'info>,

    /// mintkey to bet
    pub mint: Box<Account<'info, Mint>>,

    /// will hold the spl of the raffles tickets price
    /// CHECK: it's alright
    #[account(
        mut, 
        constraint = vault.mint == mint.key(),
        constraint = vault.owner == creator.key(),
    )]
    pub vault: Box<Account<'info, TokenAccount>>,

    pub rent_sysvar: Sysvar<'info, Rent>,
    pub clock_sysvar: Sysvar<'info, Clock>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateLotteryAccount<'info> {
    /// lottery info
    #[account(
        mut,
        constraint = lottery.creator == creator.key(),
    )]
    pub lottery: Box<Account<'info, state::Lottery>>,

    /// the creator of the lottery
    #[account(mut)]
    pub creator: Signer<'info>,

    pub rent_sysvar: Sysvar<'info, Rent>,
    pub clock_sysvar: Sysvar<'info, Clock>,
    pub system_program: Program<'info, System>,
}

/// create ticket
#[derive(Accounts)]
pub struct CreateTicketAccount<'info> {
    pub lottery: Box<Account<'info, state::Lottery>>,
    /// the buyer of the ticket
    #[account(
        init,
        payer = buyer,
        space = state::Ticket::LEN,
        seeds = [
            utils::TICKET_PREFIX.as_bytes(), 
            lottery.key().as_ref(),
            buyer.key().as_ref(), 
        ], 
        bump,
    )]
    pub ticket: Box<Account<'info, state::Ticket>>,
    /// the buyer of the ticket
    #[account(mut)]
    pub buyer: Signer<'info>,

    pub system_program: Program<'info, System>,
    pub clock_sysvar: Sysvar<'info, Clock>,
}

/// buy ticket with sol
#[derive(Accounts)]
pub struct BuyTicketWithSolAccount<'info> {
    #[account(mut)]
    pub lottery: Box<Account<'info, state::Lottery>>,
    /// the buyer of the ticket
    #[account(
        mut,
        seeds = [
            utils::TICKET_PREFIX.as_bytes(), 
            lottery.key().as_ref(),
            buyer.key().as_ref(), 
        ], 
        bump,
    )]
    pub ticket: Box<Account<'info, state::Ticket>>,
    /// the buyer of the ticket
    #[account(mut)]
    pub buyer: Signer<'info>,

    /// will hold the sol of the raffles tickets price
    /// CHECK: it's alright
    #[account(mut)]
    pub vault: AccountInfo<'info>,
    /// CHECK: it's alright
    pub discount: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
    pub clock_sysvar: Sysvar<'info, Clock>,
}

/// buy ticket with spl
#[derive(Accounts)]
pub struct BuyTicketWithSplAccount<'info> {
    #[account(mut)]
    pub lottery: Box<Account<'info, state::Lottery>>,
    /// the buyer of the ticket
    #[account(
        mut,
        seeds = [
            utils::TICKET_PREFIX.as_bytes(), 
            lottery.key().as_ref(),
            buyer.key().as_ref(), 
        ], 
        bump = ticket.bump,
    )]
    pub ticket: Box<Account<'info, state::Ticket>>,
    /// the buyer of the ticket
    #[account(mut)]
    pub buyer: Signer<'info>,
    /// will hold the spl of the raffles tickets price
    /// CHECK: it's alright
    #[account(
        mut, 
        constraint = Some(vault.mint) == lottery.mint,
        constraint = vault.owner == lottery.creator,
    )]
    pub vault: Box<Account<'info, TokenAccount>>,
    /// CHECK: it's alright
    pub discount: AccountInfo<'info>,
    #[account(
        mut,
        constraint = Some(buyer_token_account.mint) == lottery.mint,
        constraint = buyer_token_account.owner == buyer.key(),
    )]
    pub buyer_token_account: Box<Account<'info, TokenAccount>>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub clock_sysvar: Sysvar<'info, Clock>,
}

/// close the lottery
/// check if the time passed the end date
#[derive(Accounts)]
pub struct CloseLottery<'info> {
    #[account(mut)]
    pub lottery: Box<Account<'info, state::Lottery>>,

    pub clock_sysvar: Sysvar<'info, Clock>,
}

/// close the account
/// check if the time passed the end date
#[derive(Accounts)]
pub struct CloseAccount<'info> {
    pub lottery: Box<Account<'info, state::Lottery>>,
    #[account(mut)]
    /// CHECK: it's alright
    pub ticket: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: it's alright
    pub receiver: AccountInfo<'info>,
}

/// create discount
#[derive(Accounts)]
pub struct CreateDiscountAccount<'info> {
    pub lottery: Box<Account<'info, state::Lottery>>,
    #[account(
        init,
        payer = creator,
        space = state::Discount::LEN,
        seeds = [
            utils::DISCOUNT_PREFIX.as_bytes(), 
            lottery.key().as_ref(),
            creator.key().as_ref(), 
        ], 
        bump,
    )]
    pub discount: Box<Account<'info, state::Discount>>,
    /// the creator of the ticket
    #[account(mut)]
    pub creator: Signer<'info>,
    pub clock_sysvar: Sysvar<'info, Clock>,
    pub system_program: Program<'info, System>,
}

/// update discount
#[derive(Accounts)]
pub struct UpdateDiscountAccount<'info> {
    pub lottery: Box<Account<'info, state::Lottery>>,
    #[account(
        mut,
        seeds = [
            utils::DISCOUNT_PREFIX.as_bytes(), 
            lottery.key().as_ref(),
            creator.key().as_ref(), 
        ], 
        bump,
    )]
    pub discount: Box<Account<'info, state::Discount>>,
    /// the creator of the ticket
    #[account(mut)]
    pub creator: Signer<'info>,
    pub clock_sysvar: Sysvar<'info, Clock>,
    pub system_program: Program<'info, System>,
}
