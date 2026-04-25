use anchor_lang::prelude::*;




#[account]

pub struct ProposalCountryPda{

    pub country_id: u16,

    pub country_name: [u8;32],

    pub country_pda_threshold : u8,

    pub approvals:  u8,

    pub approved : bool,

    pub executed: bool,

    pub total_authority: u8,

    pub initialize_authority_count : u8,  

    pub bump: u8,

} 

impl ProposalCountryPda {
     pub const SIZE: usize  =   2 +   // country_id
        32 +  // country_name
        1 +   // threshold
        1 +   // approvals
        1 +   // approved
        1 +   // executed
        1 +   // total_authority
        1 +   // initialize_authority_count
        1;    // bump
}