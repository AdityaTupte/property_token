use anchor_lang::prelude::*;



#[account]

pub struct PropertyAccountMetadata{
            
    pub property: Pubkey,

    pub legal_doc_hash: [u8; 32],

    pub last_updated: i64,
    
    pub bump: u8,

}

impl PropertyAccountMetadata {

    pub const SIZE: usize = 32 + 32+ 8 + 1 ;
    
}