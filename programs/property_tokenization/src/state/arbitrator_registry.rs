use anchor_lang::prelude::*;

use crate::traits::AuthorityRegistry;




#[account]

pub struct ArbitratorRegistry{

pub property_system_account: Pubkey,        // Property system this arbitrator registry belongs to

pub current_number_of_arbitrators: u8,             // Current number of arbitrators in the list

pub total_arbitrators: u8,                  // Number of active arbitrators

pub vote_threshold: u8,                         // Minimum approvals required for arbitration decisions

pub claim_deadline_ts : i64,

pub bump: u8,                              // PDA bump seed for address derivation


}

impl ArbitratorRegistry {
    
    // pub const SIZE : usize = 32 + 4 + (MAX_ARBITRATOR  * 32) +  1 + 1 + 8 +1 ; 
    pub const SIZE : usize = 32 +  1 + 1  +1 + 8 +1 ; 
}

impl AuthorityRegistry for ArbitratorRegistry  {
    
    fn total_authority(&mut self)-> &mut u8 {
        
        &mut self.total_arbitrators

    }

    fn current_authority(&mut self)-> &mut u8 {
        &mut self.current_number_of_arbitrators
    }

}

