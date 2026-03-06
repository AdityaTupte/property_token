use anchor_lang::prelude::*;

use crate::common::{ MAX_COUNTRY_APPROVE_AUTHORITY, MAX_COUNTRY_AUTHORITY, MAX_COUNTRY_NAME};




#[account]

pub struct ProposalCountryPda{

    pub country_id: u16,

    pub country_name: String,

    pub country_pda_threshold : u8,

    pub authority:  Vec<Pubkey>, 
    
    pub approvals:  Vec<Pubkey>,

    pub approved : bool,

    pub executed: bool,           

    pub bump: u8,

}

impl ProposalCountryPda {
     pub const SIZE: usize  = 2 + (4 + MAX_COUNTRY_NAME) + (4 + (32 * MAX_COUNTRY_AUTHORITY)) +  (4 + (32 * MAX_COUNTRY_APPROVE_AUTHORITY)) + 1 + 1 + 1 ;
}