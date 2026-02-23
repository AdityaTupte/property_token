use anchor_lang::prelude::*;

#[account]
pub struct VoterReciept{

    pub proposal: Pubkey,
    pub voter: Pubkey,
    pub voting_power : u64,
    pub vote : bool,
    pub bump: u8,


}

impl VoterReciept {
    pub const SIZE:usize= 32 + 32 + 8 + 1 + 1 ;
}