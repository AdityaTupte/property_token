use anchor_lang::prelude::*;
pub const MAX_STATE_NAME: usize = 32;
pub const MAX_STATE_AUTHORITIES: usize = 10;
pub const MAX_APPROVAL: usize = 10;


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
                    2 +    
                    4 + 
                    MAX_STATE_NAME + 
                    4 + (MAX_STATE_AUTHORITIES * 32 ) + 
                    1 + 
                    2 +
                    32 + 
                    4 + (32 * MAX_APPROVAL) +
                    1 +
                    1 +
                    1 ;  
    
}
