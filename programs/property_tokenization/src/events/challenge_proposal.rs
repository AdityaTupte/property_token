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