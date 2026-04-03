use anchor_lang::prelude::*;

use crate::{common::MAX_ARBITRATOR, constant::AuthorityRegistry};


#[account]

pub struct ArbitratorRegistry{

pub property_system_account: Pubkey,        // Property system this arbitrator registry belongs to

pub arbitrator:  Vec<Pubkey>,  // Fixed list of arbitrator public keys

pub total_arbitrator: u8,                  // Number of active arbitrators

pub threshold: u8,                         // Minimum approvals required for arbitration decisions

pub claim_deadline_ts : i64,

pub bump: u8,                              // PDA bump seed for address derivation


}

impl ArbitratorRegistry {
    
    pub const SIZE : usize = 32 + 4 + (MAX_ARBITRATOR  * 32) +  1 + 1 + 8 +1 ; 
}

impl AuthorityRegistry for ArbitratorRegistry  {
    
    fn registry(&mut self)-> &mut Vec<Pubkey>{

        &mut self.arbitrator

    }

}

