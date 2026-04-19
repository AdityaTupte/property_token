use anchor_lang::prelude::*;

use crate::{common::MAX_TRUSTEES, constant::AuthorityRegistry};


#[account]

pub struct TrusteeRegistry{

pub property_system_account: Pubkey,      // Property system this trustee registry belongs to

pub current_number_of_trustees: u8,             // Current number of trustees in the list

pub total_trustees: u8,                  // Number of active trustees in the list

pub vote_threshold: u8,                       // Minimum approvals required for a valid decision

pub claim_deadline_ts : i64,

pub bump: u8,                            // PDA bump seed for address derivation

}

impl TrusteeRegistry {
    
    // pub const SIZE : usize = 32 + 4 + (MAX_TRUSTEES  * 32) +  1 + 1 + 8 + 1 ; 
    pub const SIZE : usize = 32 + 1 +  1 + 1 + 8 + 1 ;
}

// impl AuthorityRegistry for TrusteeRegistry  {
    
//     fn registry(&mut self)-> &mut Vec<Pubkey>{

//         &mut self.trustees

//     }

// }

