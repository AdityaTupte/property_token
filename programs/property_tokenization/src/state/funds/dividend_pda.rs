use anchor_lang::prelude::*;

#[account]
pub struct DividendPda {
    pub property_system: Pubkey,
    pub dividend_per_token:u128,
    pub last_updated_ts: i64,
    pub bump: u8,
}

impl DividendPda{

    pub const SIZE : usize = 32 + 16 + 8 + 1 ;

}