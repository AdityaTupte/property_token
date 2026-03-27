use anchor_lang::prelude::*;

use crate::common::{AuthorityType, ProposalStatus, ProposalType};


pub trait BaseProposal {
    fn proposal_id(&mut self) -> &mut u64;

    fn merkle_root(&mut self) -> &mut [u8; 32];

    fn arbitrar_list(&mut self) -> &mut Vec<Pubkey>;

    fn arbitrar_approved(&mut self) -> &mut bool;

    fn proposal_status(&mut self) -> &mut ProposalStatus;

    fn snapshot_submitted(&mut self) -> &mut bool;

    fn slot(&mut self) -> &mut u64;

    fn bump(&mut self) -> &mut u8;
}


pub trait Governance: BaseProposal {
    fn start_time(&mut self) -> &mut i64;

    fn end_time(&mut self) -> &mut i64;

    fn total_voting_power(&mut self) -> &mut u64;

    fn vote_threshold(&mut self) -> &mut u64;

    fn votes_for(&mut self) -> &mut u64;

    fn votes_against(&mut self) -> &mut u64;

    fn proposal_type(&mut self) -> &mut ProposalType;

    fn deadline(&mut self) -> &mut i64;
}

pub trait AuthorityGovernance: BaseProposal {

    fn is_finalize(&mut self)-> &mut bool;

    fn new_authority(&mut self)-> &mut Vec<Pubkey>;

    fn proposal_type(&mut self) -> &mut AuthorityType;

    fn authority_to_resign(&mut self) -> &mut Vec<Pubkey>;

    fn candidate_submission_deadline(&mut self) -> &mut i64;

    fn voting_for_authority_deadline(&mut self) -> &mut i64;

    fn add_new_authority_deadline(&mut self) -> &mut i64;

    fn challenge_new_authority_deadline(&mut self) -> &mut i64;

    fn index_for_removal(&mut self) ->  &mut u8 ;
}


pub trait AuthorityRegistry {
     fn registry(&mut self)-> &mut Vec<Pubkey>;
}


// pub trait Governance {

//     fn proposal_id(&mut self) -> &mut u64;

//     fn start_time(&mut self) -> &mut i64; 

//     fn end_time(&mut self) -> &mut i64;

//     fn merkle_root(&mut self) -> &mut [u8;32] ;

//     fn arbitrar_list(&mut self) -> &mut Vec<Pubkey>;

//     fn arbitrar_approved(&mut self)-> &mut bool;

//     fn total_voting_power(&mut self) -> &mut u64;

//     fn vote_threshold(&mut self) -> &mut u64;

//     fn votes_for(&mut self) -> &mut u64;

//     fn votes_against(&mut self) -> &mut u64;

//     fn proposal_status(&mut self) -> &mut ProposalStatus;
    
//     fn snapshot_submitted(&mut self) -> &mut bool;

//     fn proposal_type(&mut self) -> &mut ProposalType;

//     fn slot(&mut self) -> &mut u64;

//     fn deadline(&mut self) -> &mut i64;

//     fn bump(&mut self) -> &mut u8;
        
// }

pub trait Receipt  {

    fn proposal(&mut self) -> &mut Pubkey;

    fn voter(&mut self) -> &mut Pubkey;

    fn voting_power(&mut self) ->&mut u64;

    fn bump(&mut self) -> &mut u8 ;

}


// pub trait AuthorityGovernance {
//     fn proposal_id(&mut self) -> &mut u64;

//     fn merkle_root(&mut self) -> &mut [u8;32] ;

//     fn arbitrar_list(&mut self) -> &mut Vec<Pubkey>;

//     fn arbitrar_approved(&mut self)-> &mut bool;

//     fn proposal_status(&mut self) -> &mut ProposalStatus;
    
//     fn snapshot_submitted(&mut self) -> &mut bool;

//     fn slot(&mut self) -> &mut u64;

//     fn bump(&mut self) -> &mut u8;

//     fn candidate_submission_deadline(&mut self) -> &mut i64;

//     fn voting_for_authority_deadline(&mut self) -> &mut i64;

//     fn add_new_authority_deadline(&mut self) -> &mut i64;

//     fn challenge_new_authority_deadline(&mut self) -> &mut i64;


// }


