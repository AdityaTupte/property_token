use anchor_lang::prelude::*;

pub const MAX_TRUSTEES: usize = 5 ;
#[account]

pub struct TrusteeRegistry{

    pub property_system_accout : Pubkey,

    pub trustees: [Pubkey; MAX_TRUSTEES],

    pub total_trustees: u8,
    
    pub threshold: u8,
    
    pub bump: u8,

}

impl TrusteeRegistry {
    
    pub const SIZE : usize = 32 + MAX_TRUSTEES  * 32 +  1 + 1 +1 ; 
}

