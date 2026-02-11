use anchor_lang::prelude::*;


#[account]

pub struct LandAccount{

    pub land_id: u64,
    pub state_id: u16,
    pub state_pubkey : Pubkey,
    pub country_id : u16,
    pub country_pubkey : Pubkey,
    pub issued_at : i64,
    pub issued_by: Pubkey,
    pub metadata: Pubkey,   
    pub bump: u8,
}


impl LandAccount {
    pub const SIZE: usize = 8 + 2 + 32 + 2 + 32 + 8 + 32 + 32 + 1   ;
    
}