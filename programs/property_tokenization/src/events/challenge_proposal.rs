use anchor_lang::prelude::*;

use crate::common::{AuthorityType, ReasonType};



#[event]

pub struct ChallengeProposalCreated {

    pub proposal_id : u64,

    pub proposal_key : Pubkey,

    pub property_system:Pubkey,

}

#[event]

pub struct ChallengeProposalOffenderAuthorityAdded {


    pub proposal_key : Pubkey,

    pub authority : Pubkey,
    
    pub authority_type : AuthorityType,


}



#[event]

pub struct ChallengeProposalExecuted {


    pub proposal_key : Pubkey,

    pub outcoome :ReasonType,


}

#[event]

pub struct FinalizeAccusedAuthority {


    pub proposal_key : Pubkey,

    pub authority : Pubkey,

    pub authority_type : AuthorityType,

    pub outcoome :ReasonType,


}


#[event]
pub struct  RemoveGuiltyAuthority{

    pub challenge_proposal_key : Pubkey,

    pub removal_guilty_authority_proposal : Pubkey,

    pub authority_type : AuthorityType,



}


#[event]

pub struct  AddedAuthorityForRemoval{

    pub proposal_key : Pubkey,

    pub property_system : Pubkey,

    pub authority : Pubkey,

    pub authority_type : AuthorityType,


}


#[event]

pub struct  SubmitSnapshotForRemoveAuthority{

    pub proposal_key :Pubkey,

    pub candidate_submision_deadline : i64,
    
    pub voting_for_authority_deadline : i64,
    
    pub add_new_authority_deadline : i64,
    
    pub challenge_new_authority_deadline : i64,

}

#[event]

pub struct AuthorityRevise{

    pub proposal_key :Pubkey,

    pub new_authority :Pubkey,

    pub old_authority :Pubkey


}


#[event]
pub struct ResignationOfAuthority{


   pub proposal:Pubkey,
   pub authority:Pubkey,


}


#[event]
pub struct FinalizeAuthority{
    pub proposal:Pubkey,
    pub property_system:Pubkey,
     pub authority:Pubkey,
    pub authority_type:AuthorityType,
}