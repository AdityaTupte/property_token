use anchor_lang::prelude::*;




#[account()]
pub struct TrusteeRecepit {

    pub property_system_account: Pubkey,

    pub trustee: Pubkey,

    pub bump: u8,

}


impl TrusteeRecepit {
    
    pub const SIZE : usize = 32 + 32 + 1 ;    
}