use anchor_lang::prelude::*;

use crate::common::AuthorityType;


#[account]
pub struct OffenderReceipt{


    pub offender_key : Pubkey,

    pub proposal_key :Pubkey,

    pub property_system_key : Pubkey,

    pub authority_type: AuthorityType,

    pub is_finalized : bool,

    pub bump:u8,
}

impl OffenderReceipt {
    pub const SIZE :usize= 32 + 32+ 32 +1 +1 +1 ;
}