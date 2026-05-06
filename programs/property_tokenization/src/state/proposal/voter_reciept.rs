use anchor_lang::prelude::*;

use crate::traits::{Receipt};

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


impl Receipt for VoterReciept  {
    
    fn voter(&mut self) -> &mut Pubkey {
        return &mut self.voter;
    }

    fn voting_power(&mut self) ->&mut u64 {
        return &mut self.voting_power;
    }

    fn proposal(&mut self) -> &mut Pubkey {
        return &mut self.proposal;
    }

    fn bump(&mut self) -> &mut u8 {
        return &mut self.bump;
    }
}