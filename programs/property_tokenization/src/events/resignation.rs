use anchor_lang::prelude::*;

use crate::common::AuthorityType;

#[event]

pub struct ResignationCreated{

    pub proposal:Pubkey,

    pub authority:Pubkey,

    pub authority_type: AuthorityType,

}



#[event]

pub struct ApproveByAuthority{

    pub proposal:Pubkey,

    pub authority:Pubkey,


}