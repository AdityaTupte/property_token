use anchor_lang::prelude::*;

#[account]

pub struct TreasuryPda{

    pub property_system_accout: Pubkey, 

    pub last_distribution_ts: i64,
    
    pub bump : u8,

}

impl TreasuryPda {
    pub const SIZE : usize =  32 + 32 +  1; 
}
