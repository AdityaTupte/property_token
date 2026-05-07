use anchor_lang::prelude::*;

use crate::{common::{ ProposalStatus, ProposalType}, traits::{BaseProposal, Governance},};


#[account]
pub struct TokenTransferProposal{

    pub proposal_id : u64,

    pub amount_required : u64,

    pub reason_hash : [u8; 32] ,

    pub property_system : Pubkey,

    pub recepient_wallet : Pubkey,

    pub merkle_root: [u8; 32],

    pub arbitrar_approvals_count: u8,

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

    pub deadline: i64,

    pub slot : u64,

    pub bump : u8,

}impl TokenTransferProposal {
    pub const SIZE: usize =
        8 +  // proposal_id
        8 +  // amount_required
        32 + // reason_hash
        32 + // property_system
        32 + // recepient_wallet
        32 + // merkle_root
        1 +  // arbitrar_approvals_count
        1 +  // is_arbitrar_approved
        8 +  // total_voting_power
        8 +  // votes_for
        8 +  // votes_against
        8 +  // vote_threshold
        8 +  // start_time
        8 +  // end_time
        1 +  // status
        1 +  // snapshot_submitted
        1 +  // proposal_type
        8 +  // deadline
        8 +  // slot
        1;   // bump

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


    }             
 }
impl BaseProposal for TokenTransferProposal {

    fn proposal_id(&mut self) -> &mut u64 {
        &mut self.proposal_id
    }

    fn merkle_root(&mut self) -> &mut [u8; 32] {
        &mut self.merkle_root
    }

    fn arbitrar_total_count(&mut self) -> &mut u8 {
        &mut self.arbitrar_approvals_count
    }

    fn arbitrar_approved(&mut self) -> &mut bool {
        &mut self.is_arbitrar_approved
    }

    fn proposal_status(&mut self) -> &mut ProposalStatus {
        &mut self.status
    }

    fn snapshot_submitted(&mut self) -> &mut bool {
        &mut self.snapshot_submitted
    }

    fn slot(&mut self) -> &mut u64 {
        &mut self.slot
    }

    fn bump(&mut self) -> &mut u8 {
        &mut self.bump
    }
}


impl Governance for TokenTransferProposal {

    fn start_time(&mut self) -> &mut i64 {
        &mut self.start_time
    }

    fn end_time(&mut self) -> &mut i64 {
        &mut self.end_time
    }

    fn total_voting_power(&mut self) -> &mut u64 {
        &mut self.total_voting_power
    }

    fn vote_threshold(&mut self) -> &mut u64 {
        &mut self.vote_threshold
    }

    fn votes_for(&mut self) -> &mut u64 {
        &mut self.votes_for
    }

    fn votes_against(&mut self) -> &mut u64 {
        &mut self.votes_against
    }

    fn proposal_type(&mut self) -> &mut ProposalType {
        &mut self.proposal_type
    }

    fn deadline(&mut self) -> &mut i64 {
        &mut self.deadline
    }
}





//  impl Governance for SafetyProposal {

//     fn proposal_id(&mut self) ->&mut u64 {
//         return &mut self.proposal_id;
//     }

//     fn start_time(&mut self) -> &mut i64 {
//         return &mut self.start_time;
//     }

//     fn end_time(&mut self) -> &mut i64 {
//         return &mut self.end_time;
//     }
//     fn merkle_root(&mut self) -> &mut [u8;32] {
//         return &mut self.merkle_root;
//     }

//     fn arbitrar_list(&mut self) -> &mut Vec<Pubkey> {
//         return  &mut self.arbitrar_approvals;
//     }
    
//     fn arbitrar_approved(&mut self)-> &mut bool {
//         return  &mut self.is_arbitrar_approved ;
//     }

//     fn total_voting_power(&mut self) -> &mut u64 {
//         return &mut self.total_voting_power;
//     }
//     fn vote_threshold(&mut self) -> &mut u64 {
//         return &mut self.vote_threshold;
//     }

//     fn votes_for(&mut self) -> &mut u64 {
//         return &mut self.votes_for;
//     }

//     fn votes_against(&mut self) -> &mut u64 {
//         return &mut self.votes_against;
//     }

//     fn proposal_status(&mut self) -> &mut ProposalStatus {
//         return &mut self.status;
//     }

//     fn snapshot_submitted(&mut self) -> &mut bool {
//         return &mut self.snapshot_submitted;
//     }

//     fn proposal_type(&mut self) -> &mut ProposalType {
//         return &mut self.proposal_type;
//     }

//     fn slot(&mut self) -> &mut u64 {
//         return &mut self.slot;
//     }

//     fn deadline(&mut self) -> &mut i64 {
//         return &mut self.deadline;
//     }

//     fn bump(&mut self) -> &mut u8 {
//         return &mut self.bump;
//     }

// }



