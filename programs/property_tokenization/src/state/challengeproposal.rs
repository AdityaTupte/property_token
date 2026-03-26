use anchor_lang::prelude::*;

use crate::common::{AuthorityType, MAX_TRUSTEES, ProposalStatus, ProposalType, ReasonType};



#[account]

pub struct ChallengeProposal{

    pub creator:Pubkey,

    pub proposal_id : u64,

    pub property_system : Pubkey,

    pub authority_type : AuthorityType,

    pub against:Vec<Pubkey>,

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

    pub const SIZE :usize = 32 +
                            8 +
                            32 +
                            4 + (32*MAX_TRUSTEES) +
                            8 +
                            8 +
                            8 +
                            8 +
                            1 +
                            1 +
                            8 +
                            1 +
                            8 +
                            8 +
                            8 +
                            1 +
                            1 ;
                        }