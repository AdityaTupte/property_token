use anchor_lang::prelude::*;

use crate::common::AuthorityType;

#[event]
pub struct  SalaryClaimed{

    pub property_system:Pubkey,

    pub authority:Pubkey,

    pub authority_type:AuthorityType,

}

