use anchor_lang::prelude::*;

use crate::{common::{AuthorityType, ProposalStatus}, constant::{AuthorityGovernance, BaseProposal}};

#[account]
pub struct ElectAuthority{

    pub property_system: Pubkey,

    pub total_authority_to_resign : u8,

    // pub new_authority : Vec<Pubkey>,

    pub authority_type : AuthorityType,
    
    pub status:ProposalStatus,

    pub proposal_id : u64,

    pub merkle_root: [u8; 32],

    pub arbitrar_approvals_count: u8,

    pub is_initialized : bool,

    pub is_arbitrar_approved : bool,
   
    pub snapshot_submitted : bool,

    pub candidate_submission_deadline: i64,

    pub voting_for_authority_deadline : i64,

    pub add_new_authority_deadline : i64,

    pub challenge_new_authority_deadline : i64,

    pub is_finalize : bool,

    pub slot : u64,

    pub bump:u8,

}

impl ElectAuthority  {
        pub const SIZE:usize = 
                                32 +
                                // 4 + (32 * MAX_TRUSTEES) +
                                1 +
                                1 +
                                1 +
                                1 +
                                8 +
                                32 +
                                1 +
                                1 +
                                1 +
                                1 +
                                8 +
                                8 +
                                8 +
                                8 +
                                1 +
                                8 +
                                1  ;

}

// impl AuthorityGovernance for ElectAuthority {
    
//     fn proposal_id(&mut self) -> &mut u64 {
//         return &mut self.proposal_id;
//     }

//     fn merkle_root(&mut self) -> &mut [u8;32] {
//         return  &mut self.merkle_root;
//     }

//     fn arbitrar_list(&mut self) -> &mut Vec<Pubkey> {
//         return &mut self.arbitrar_approvals;
//     }

//     fn arbitrar_approved(&mut self)-> &mut bool {
//         return &mut self.is_arbitrar_approved;
//     }

//     fn proposal_status(&mut self) -> &mut ProposalStatus {
//         return &mut self.status;
//     }

//     fn snapshot_submitted(&mut self) -> &mut bool {
//         return &mut self.snapshot_submitted;
//     }
    
//     fn slot(&mut self) -> &mut u64 {
//         return &mut self.slot ;
//     }

//     fn add_new_authority_deadline(&mut self) -> &mut i64 {
//         return &mut self.add_new_authority_deadline;
//     }

//     fn candidate_submission_deadline(&mut self) -> &mut i64 {
//         return &mut self.candidate_submission_deadline;
//     }

//     fn challenge_new_authority_deadline(&mut self) -> &mut i64 {
//         return &mut self.challenge_new_authority_deadline;
//     }

//     fn voting_for_authority_deadline(&mut self) -> &mut i64 {
//         return &mut self.voting_for_authority_deadline;
//     }

//     fn bump(&mut self) -> &mut u8 {
//         return &mut self.bump;
//     }   

// }

impl BaseProposal for ElectAuthority {

    fn proposal_id(&mut self) -> &mut u64 {
        &mut self.proposal_id
    }

    fn merkle_root(&mut self) -> &mut [u8; 32] {
        &mut self.merkle_root
    }

    fn arbitrar_total_count(&mut self) -> &mut u8 {
        &mut self.arbitrar_approvals_count
    }

    // fn arbitrar_list(&mut self) -> &mut Vec<Pubkey> {
    //     &mut self.arbitrar_approvals
    // }

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

impl AuthorityGovernance for ElectAuthority {

    // fn new_authority(&mut self)-> &mut Vec<Pubkey>{
    //     &mut self.new_authority
    // }

    fn is_finalize(&mut self) -> &mut bool{
        &mut self.is_finalize
    }

    fn proposal_type(&mut self) -> &mut AuthorityType {
        &mut self.authority_type
    }

    fn total_authority_to_resign(&mut self) -> &mut u8{
        &mut self.total_authority_to_resign
    }

    fn add_new_authority_deadline(&mut self) -> &mut i64 {
        &mut self.add_new_authority_deadline
    }

    fn candidate_submission_deadline(&mut self) -> &mut i64 {
        &mut self.candidate_submission_deadline
    }

    fn challenge_new_authority_deadline(&mut self) -> &mut i64 {
        &mut self.challenge_new_authority_deadline
    }

    fn voting_for_authority_deadline(&mut self) -> &mut i64 {
        &mut self.voting_for_authority_deadline
    }

    
}