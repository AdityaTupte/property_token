use anchor_lang::prelude::*;

use crate::common::{AuthorityType, ProposalStatus};


#[account()]

pub struct Resignation{

    pub authority: Pubkey,

    pub property_system: Pubkey,

    pub authority_type : AuthorityType,

    pub time : i64,

    pub status:ProposalStatus,

    pub bump : u8,

}

impl Resignation {
    
    pub const SIZE:usize = 
                        32 +
                        32 +
                        1  +
                        8 +
                        1 ;

}