use anchor_lang::prelude::*;

pub const MAX_AUTHORITY :usize = 10;
#[account]
pub  struct ApproveCountryAuthority{

    pub authority : Vec<Pubkey>,

    pub threshold : u8,

    pub bump:u8,

}

impl ApproveCountryAuthority {
    pub const SIZE : usize =  4  + ( MAX_AUTHORITY * 32) + 1 + 1; 
}