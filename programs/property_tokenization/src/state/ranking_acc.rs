use anchor_lang::prelude::*;


#[account]

pub struct RankingAccount{


    pub candidate_key : Pubkey,

    pub rank: u8,

    pub elect_proposal:Pubkey,

    pub bump:u8,
    

}

impl RankingAccount {
    
    pub const SIZE:usize= 32 + 1 +32 + 1;
}