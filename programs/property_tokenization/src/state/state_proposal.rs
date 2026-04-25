use anchor_lang::prelude::*;




#[account]

pub struct  StateProposalPda{

    pub state_id : u16,

    pub state_name: [u8;32],

    pub state_total_authorities : u8,

    // pub state_authorities : Vec<Pubkey>,

    pub state_authority_threshold: u8,

    pub country_id: u16,

    pub country_pubkey : Pubkey,

    // pub approval:Vec<Pubkey>,
     pub approval:u8,

    pub approved: bool,

    pub executed: bool,
    
    pub bump : u8,

}

impl StateProposalPda {
    pub const SIZE: usize =
        2 +   // state_id (u16)
        32 +  // state_name [u8;32]
        1 +   // total_authorities
        1 +   // threshold
        2 +   // country_id
        32 +  // country_pubkey
        1 +   // approval
        1 +   // approved
        1 +   // executed
        1;    // bump
}

#[account]
pub struct  StateProposalAprroveReceipt{

    pub proposal:Pubkey,

    pub bump :u8,

}

impl StateProposalAprroveReceipt{


    pub const SIZE:usize = 32 +1 ;


}