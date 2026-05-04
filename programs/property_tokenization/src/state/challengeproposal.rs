use anchor_lang::prelude::*;

use crate::common::{  ProposalStatus, ProposalType, ReasonType};



#[account]

pub struct ChallengeProposal{

    pub creator:Pubkey,

    pub proposal_id : u64,

    pub property_system : Pubkey,

    // pub authority_type : AuthorityType,

    pub trustee_offender_total_number:u8,

    pub arbitrar_offender_total_number:u8,

    pub required_vote_to_active : u64,

    pub vote_gained : u64,

    pub charges_hash: [u8; 32], 
    
    pub evidence_hash : [u8;32],

    pub proposal_type : ProposalType,

    pub status: ProposalStatus,

    pub merkle_root : [u8;32],

    pub guilty : ReasonType, 

    pub created_at: i64,

    pub result_time : i64,
    
    pub voting_deadline: i64,

    pub index : u8,

    pub bump : u8,


}

impl ChallengeProposal{

    pub const SIZE: usize =
    32 + // creator
    8  + // proposal_id
    32 + // property_system
    1  + // trustee_offender_total_number
    1  + // arbitrar_offender_total_number
    8  + // required_vote_to_active
    8  + // vote_gained
    32 + // charges_hash
    32 + // evidence_hash
    1  + // proposal_type
    1  + // status
    32 + // merkle_root
    1  + // guilty
    8  + // created_at
    8  + // result_time
    8  + // ✅ voting_deadline (MISSING)
    1  + // index
    1;   // bump
                        }