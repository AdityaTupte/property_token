use anchor_lang::prelude::*;

#[account]
pub struct ReinvestmentPda{

    pub property_system: Pubkey,
    
    pub reinvestement_used:u64,

    pub bump: u8,


}

impl ReinvestmentPda{

    pub const SIZE:usize = 32 + 8 + 1 ;


}


