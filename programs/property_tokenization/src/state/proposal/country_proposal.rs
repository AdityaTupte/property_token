use anchor_lang::prelude::*;

use crate::common::{ MAX_COUNTRY_APPROVE_AUTHORITY, MAX_COUNTRY_AUTHORITY, MAX_COUNTRY_NAME};




#[account]

pub struct ProposalCountryPda{

    pub country_id: u16,

    pub country_name: String,

    pub country_pda_threshold : u8,

    pub approvals:  u8,

    pub approved : bool,

    pub executed: bool,

    pub total_authority: u8,

    pub initialize_authority_count : u8,  

    pub bump: u8,

} 

impl ProposalCountryPda {
     pub const SIZE: usize  = 2 + (4 + MAX_COUNTRY_NAME) + 1  + 1 + 1 + 1 + 1+ 1 + 1 ;
}