use anchor_lang::prelude::*;

#[account]
pub struct DividendPda {
    pub property_system: Pubkey,
    pub last_distribution_ts: i64,
    pub bump: u8,
}

impl DividendPda{

    pub const SIZE : usize = 32 + 8 + 1 ;

}