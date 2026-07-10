use anchor_lang::prelude::*;

use crate::common::{AuthorityType, ProposalStatus};

#[event]

pub struct FinalizeProposal{

    pub proposal : Pubkey,

    pub proposal_status : ProposalStatus,

}


#[event]

pub struct DeleteProposal{

    pub proposal : Pubkey,

    pub deleted_by : Pubkey,

}


#[event]

pub struct CandidateSubmitedForProposal{

     pub proposal : Pubkey,

     pub candidate:Pubkey,

     pub authority_type : AuthorityType,
}



#[event]

pub struct AddedNewAuthority{

    pub proposal : Pubkey,

     pub candidate:Pubkey,

     pub authority_type : AuthorityType,

     pub rank:u8,

}
 


 #[event]
 pub struct TreasuryDistributionEvent{
  pub  property_system:Pubkey,
   pub  time:i64
 }