use anchor_lang::prelude::*;



#[account]

pub struct LandAccountMetadata{
            
    pub land: Pubkey,

    pub legal_doc_hash: [u8; 32],

    pub last_updated: i64,
    
    pub bump: u8,

}

impl LandAccountMetadata {

    pub const SIZE: usize = 32 + 32+ 8 + 1 ;
    
}