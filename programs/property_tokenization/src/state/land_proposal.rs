use anchor_lang::prelude::*;

pub const MAX_APPROVAL:usize =10 ;
#[account]

pub struct LandProposal{

    pub land_id : u64,

    pub state_id : u16,

    pub state_pubkey : Pubkey,

    pub country_id : u16,

    pub country_pubkey : Pubkey,

    pub legal_doc_hash: [u8; 32],

    pub issused_by : Pubkey,

    pub approval : Vec<Pubkey>,

    pub approved : bool,

    pub executed : bool,

    pub bump : u8,
}

impl LandProposal {
    pub const SIZE:usize = 
                8 +
                2 +
                32 +
                2 + 
                32 +
                32 +
                32 +
                4 + (32 * 10) + 
                1 +
                1 +
                1 ;
}