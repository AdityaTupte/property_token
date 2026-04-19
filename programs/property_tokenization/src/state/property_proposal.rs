use anchor_lang::prelude::*;



pub const MAX_APPROVAL:usize =10 ;
#[account]

pub struct PropertyProposal{

    pub property_id : u64,

    pub property_system_pubkey : Pubkey,

    pub state_id : u16,

    pub state_pubkey : Pubkey,

    pub country_id : u16,

    pub country_pubkey : Pubkey,

    pub legal_doc_hash: [u8; 32],

    pub issued_by : Pubkey,

    pub approval_count :u8,

    pub approved : bool,

    pub executed : bool,

    pub bump : u8,
}

impl PropertyProposal {
    pub const SIZE:usize = 
                8 +
                32+
                2 +
                32 +
                2 + 
                32 +
                32 +
                32 +
               8 + 
                1 +
                1 +
                1 ;
}


#[account]
pub struct PropertyProposalReceipt {

    pub proposal_pubkey : Pubkey,

    pub bump : u8,

}

impl PropertyProposalReceipt {
  pub  const SIZE:usize = 32 +1 ;
}