use anchor_lang::prelude::*;

#[account]

pub struct TreasuryPda{

    pub property_system_accout : Pubkey,

    pub revenue_acc: Pubkey,

    pub reinvenstement_acc : Pubkey,

    pub safety_acc: Pubkey,


}

impl TreasuryPda {
    pub const SIZE : usize = 32 + 32 + 32 + 32; 
}
