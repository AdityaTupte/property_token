use anchor_lang::prelude::*;

pub const MAX_ARBITRATOR: usize = 5 ;
#[account]

pub struct ArbitratorRegistry{

    pub property_system_accout : Pubkey,

    pub arbitrator: [Pubkey; MAX_ARBITRATOR],

    pub total_arbitrator: u8,
    
    pub threshold: u8,
    
    pub bump: u8,

}

impl ArbitratorRegistry {
    
    pub const SIZE : usize = 32 + MAX_ARBITRATOR  * 32 +  1 + 1 +1 ; 
}

