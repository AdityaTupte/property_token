use anchor_lang::prelude::*;




#[account()]
pub struct ArbitratorRecepit {

    pub property_system_account: Pubkey,
    pub arbitrator: Pubkey,
    pub bump: u8,

}


impl ArbitratorRecepit {
    
    pub const SIZE : usize = 32 + 32 + 1 ;    
}