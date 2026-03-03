use anchor_lang::prelude::*;

use crate::{constant::{ProposalStatus, ProposalType}, state::MAX_ARBITRATOR};


#[account()]
pub struct SafetyProposal{

    pub proposal_id : u64,

    pub amount_required : u64,

    pub reason_hash : [u8; 32] ,

    pub property_system : Pubkey,

    pub recepient_wallet : Pubkey,

    pub merkle_root: [u8; 32],

    pub arbitrar_approvals: Vec<Pubkey>,

    pub is_arbitrar_approved : bool,

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

impl SafetyProposal {
    pub const SIZE:usize = 
                    
                    8 +
                    8 +
                    32 +
                    32 +
                    32 +
                    32 +
                    4 + (32 * MAX_ARBITRATOR) + 
                    1 +
                    8 +
                    8 +
                    8 +
                    8 +
                    8 +
                    8 +
                    1 + 
                    1 + 
                    1 +
                    8 +
                    1 ;

    pub fn initialize(
        &mut self,
        proposal_id:u64,
        property_system: Pubkey,
        amount_required : u64,
        reason_hash : [u8;32],
        receipent_wallet : Pubkey,
        bump :u8,
        total_voting_power : u64,
    ) {

        self.proposal_id = proposal_id;

        self.property_system = property_system;

        self.amount_required = amount_required;

        self.reason_hash = reason_hash;

        self.recepient_wallet = receipent_wallet;

        self.total_voting_power = total_voting_power;

        self.bump = bump;   

        self.status = ProposalStatus::Draft;

        self.proposal_type = ProposalType::USESAFETY;

    }


       
                    
 }