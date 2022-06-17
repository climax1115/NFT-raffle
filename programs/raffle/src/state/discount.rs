use anchor_lang::{prelude::*};

#[account]
#[derive(Debug)]
pub struct Discount {
	// discount type - 0: nft holder, 1: individual
	pub discount_type: u8,
	// candymachine creator or user wallet
	pub verifier: Pubkey,
	// discount amount percentage
	pub discount: u8,
	pub lottery: Pubkey,
	pub bump: u8,
	// discount creator
	pub creator: Pubkey,
}

impl Discount {
    pub const LEN: usize = 8 + 1 + 32 + 1 + 32 + 1 + 32;
}