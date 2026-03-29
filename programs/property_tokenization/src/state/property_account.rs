use anchor_lang::prelude::*;


#[account]

pub struct PropertyAccount{

    pub property_id: u64,
    pub is_leased : bool,
    pub property_system: Pubkey,
    pub page_number : u16,
    pub state_id: u16,
    pub state_pubkey : Pubkey,
    pub country_id : u16,
    pub country_pubkey : Pubkey,
    pub issued_at : i64,
    pub issued_by: Pubkey,
    pub metadata: Pubkey,   
    pub bump: u8,
}


impl PropertyAccount {
    pub const SIZE: usize = 8 + 1 + 32 + 2  + 2 + 32 + 2 + 32 + 8 + 32 + 32 + 1   ;
    
}