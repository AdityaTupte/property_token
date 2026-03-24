use anchor_lang::prelude::*;

use crate::common::{MAX_TRUSTEES, ProposalStatus};



#[account]

pub struct ChallengeProposal{

    pub creator:Pubkey,

    pub against:Vec<Pubkey>,

    pub required_vote_to_active : u64,

    pub charges_hash: [u8; 32], 
    
    pub evidence_hash : [u8;32],

    pub status: ProposalStatus,
    
    pub created_at: i64,
    
    pub voting_deadline: i64,

    pub bump : u8,


}

impl ChallengeProposal{

    pub const SIZE :usize = 32 +
                            4 + (32*MAX_TRUSTEES) +
                            8 +
                            8 +
                            8 +
                            1 +
                            8 +
                            8 +
                            1 ;
                        }