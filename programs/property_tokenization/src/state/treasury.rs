use anchor_lang::prelude::*;

#[account]

pub struct TreasuryPda{

    pub property_system_accout: Pubkey, // Property system this treasury belongs to


    pub reinvenstement_acc: Pubkey, // Used for reinvestment and property growth

    pub safety_acc: Pubkey, // Emergency and risk-reserve funds

    pub divdend_acc: Pubkey,
    
    pub bump : u8,

}

impl TreasuryPda {
    pub const SIZE : usize =  32 + 32 + 32 + 32 + 1; 
}
