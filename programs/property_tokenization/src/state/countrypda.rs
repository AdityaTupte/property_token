use anchor_lang::prelude::*;


pub const MAX_COUNTRY_NAME: usize = 64;

pub const MAX_COUNTRY_AUTHORITY: usize = 10;

#[account]

pub struct Country{

    pub country_id: u16,

    pub country_name: String,

    pub threshold : u8,
    
    pub authority: Vec<Pubkey>,          

    pub bump: u8,

}

impl Country {
     pub const SIZE: usize  = 2 + (4  + MAX_COUNTRY_NAME) + 1  + (4 + (32 * MAX_COUNTRY_AUTHORITY)) + 1 + 1 ;
}