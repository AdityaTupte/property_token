use anchor_lang::prelude::*;

use crate::common::MAX_COUNTRY_APPROVE_AUTHORITY;


#[account]
pub  struct ApproveCountryAuthority{

    pub authority : Vec<Pubkey>,

    pub threshold : u8,

    pub bump:u8,

}

impl ApproveCountryAuthority {
    pub const SIZE : usize =  4  + ( MAX_COUNTRY_APPROVE_AUTHORITY * 32) + 1 + 1; 
}



#[account]
pub  struct ApproveCountryAuthorityReceipt{

    pub country_proposal : Pubkey,

    pub bump:u8,

}

impl ApproveCountryAuthorityReceipt {
    pub const SIZE : usize =  32 + 1; 
}

