use anchor_lang::prelude::*;

use crate::common::MAX_TRUSTEES;

#[account]
pub struct AuthorityVoteReceipt{

    pub voter:Pubkey,

    pub proposal :Pubkey,

    pub votes : Vec<Pubkey>,

    pub voting_power : u64,

    pub is_initialized : bool,

    pub bump: u8,

}

impl AuthorityVoteReceipt{

    pub const SIZE:usize = 32 +
                            32 +
                            4 +(32* MAX_TRUSTEES) +
                            8 +
                            1 +
                            1 ;

}