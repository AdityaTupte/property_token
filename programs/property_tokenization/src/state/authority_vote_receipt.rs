use anchor_lang::prelude::*;



#[account]
pub struct AuthorityVoteReceipt{

    pub voter:Pubkey,

    pub candidate_pubkey :Pubkey,

    pub proposal :Pubkey,

    pub voting_power : u64,

    pub bump: u8,

}

impl AuthorityVoteReceipt{

    pub const SIZE:usize = 32 +
                            32 +
                            32 +
                            8 +
                            1 ;

}