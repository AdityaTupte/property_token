use anchor_lang::prelude::*;

use crate::common::AuthorityType;

#[account()]

pub struct AuthorityCandidate{

    pub candidate:Pubkey,

    pub proposal : Pubkey,

    pub property_system:Pubkey,

    pub selected : bool,

    pub selected_time : i64,

    pub vote_gained : u64,

    pub authority_type : AuthorityType,

    pub bump : u8,

}

impl AuthorityCandidate {
    pub const SIZE :usize = 
                            32 +
                            32 +
                            32 +
                            1 +
                            8 +
                            8 +
                            1 +
                            1 ;
}