use anchor_lang::prelude::*;


pub const MAX_COUNTRY_NAME1: usize = 64;
pub const MAX_COUNTRY_AUTHORITY1: usize = 10;
pub const MAX_APPROVALS1: usize = 10;

#[account]

pub struct ProposalCountryPda{

    pub country_id: u16,

    pub country_name: String,

    pub country_pda_threshold : u8,

    pub authority: Vec<Pubkey>, 
    
    pub approvals: Vec<Pubkey>,

    pub approved : bool,

    pub executed: bool,           

    pub bump: u8,

}

impl ProposalCountryPda {
     pub const SIZE: usize  = 2 + (4  + MAX_COUNTRY_NAME1) + (4 + (32 * MAX_APPROVALS1)) +  (4 + (32 * MAX_COUNTRY_AUTHORITY1)) + 1 + 1 + 1 ;
}