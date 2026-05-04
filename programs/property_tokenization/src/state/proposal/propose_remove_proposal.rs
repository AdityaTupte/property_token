use anchor_lang::prelude::*;
use crate::{common::{ ProposalStatus, ProposalType}};

#[account]

pub struct ProposeRemoveProposal{

    pub challenge_proposal : Pubkey,

    pub merkle_root: [u8; 32],

    pub total_voting_power: u64,

    pub votes_for: u64,
   
    pub votes_against: u64,

    pub vote_threshold: u64,
   
    pub start_time: i64,

    pub end_time: i64,
    
    pub status : ProposalStatus,

    pub snapshot_submitted : bool,

    pub proposal_type : ProposalType,

    pub slot : u64,

    pub bump : u8,

}


impl ProposeRemoveProposal {
    pub const SIZE: usize =
      
        32 + // Pubkey
        32 + // merkle_root
        8 +  // total_voting_power
        8 +  // votes_for
        8 +  // votes_against
        8 +  // vote_threshold
        8 +  // start_time
        8 +  // end_time
        1 +  // status (enum)
        1 +  // snapshot_submitted (bool)
        1 +  // proposal_type (enum)
        8 +  // slot
        1;   // bump
}