use anchor_lang::prelude::*;

#[account]

pub struct RankCounter{


    pub proposal_key :Pubkey,

    pub count:u8,


}

impl RankCounter {
    pub const SIZE:usize = 32 + 1; 
}