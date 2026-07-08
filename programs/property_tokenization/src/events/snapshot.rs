use anchor_lang::prelude::*;

use crate::common::{AuthorityType, ProposalType};

#[event]
pub struct SnapshotRequested {
    
    pub proposal_id : u64,

    pub proposal_key:Pubkey,

    pub mint: Pubkey,

    pub slot: u64,
}

#[event]
pub struct SnapshotRequestedForAuthority {
    
    pub proposal_id : u64,

    pub proposal_type : AuthorityType,

    pub proposal_key:Pubkey,

    pub mint: Pubkey,

    pub slot: u64,
}


#[event]
pub struct SnapshotSubmitted{

    pub proposal_id : u64,

    pub proposal_type : ProposalType,

    pub proposal_key : Pubkey,


}