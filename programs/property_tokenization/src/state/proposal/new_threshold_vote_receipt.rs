use anchor_lang::prelude::*;

#[account]
pub struct ThresholdVoteReceipt{

    pub thresholdvoted : Pubkey,

    pub bump :u8, 
}

impl ThresholdVoteReceipt {
    pub const  SIZE:usize = 32 + 8 ;
}

