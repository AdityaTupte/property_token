use anchor_lang::prelude::*;

use crate::common::{MAX_COUNTRY_AUTHORITY, MAX_STATE_AUTHORITIES, MAX_STATE_NAME};



#[account]

pub struct  StateProposalPda{

    pub state_id : u16,

    pub state_name: String,

    pub state_authorities : Vec<Pubkey>,

    pub state_authority_threshold: u8,

    pub country_id: u16,

    pub country_pubkey : Pubkey,

    pub approval:Vec<Pubkey>,

    pub approved: bool,

    pub executed: bool,
    
    pub bump : u8,

}

impl StateProposalPda {
    pub const SIZE: usize = 
                    2 +  // state_id (u16)

        4 + MAX_STATE_NAME +  // String (length prefix + data)

        4 + (32 * MAX_STATE_AUTHORITIES) + // Vec<Pubkey>

        1 +  // threshold (u8)

        2 +  // country_id (u16)

        32 + // country_pubkey

        4 + (32 * MAX_COUNTRY_AUTHORITY) + // Vec<Pubkey>

        1 +  // approved (bool)

        1 +  // executed (bool)

        1;   // bump (u8)
    
}
