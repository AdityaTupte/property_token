use anchor_lang::prelude::*;

use crate::common::{ MAX_COUNTRY_NAME};


#[account]

pub struct Country{

    pub country_id: u16,

    pub country_name: [u8;32],

    pub threshold : u8,    

    pub total_authority : u8, 

    pub bump: u8,

}

impl Country {
     pub const SIZE: usize  = 2 + (4  + MAX_COUNTRY_NAME) + 1  +1 +  1 ;
}



#[account]

pub struct CountryAuthority{

    pub country_pda: Pubkey,

    pub bump:u8,


}


impl CountryAuthority{

    pub const SIZE:usize = 32 +1 ; 
}

