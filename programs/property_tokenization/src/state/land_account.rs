use anchor_lang::prelude::*;


#[account]

pub struct LandAccount{

    pub land_id: u16,
    pub state_id: u64,
    pub country_id : u64,
    pub issued_at : i64,
    pub issued_by: Pubkey,
    pub metadata: Pubkey,   
    pub bump: u8,
}


impl LandAccount {
    pub const SIZE: usize = 4 + 8 + 8 + 8 + 32 + 32 + 1   ;
    
}