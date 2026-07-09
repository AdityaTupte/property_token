use anchor_lang::prelude::*;

use crate::common::LeaseStatus;


#[event]

pub struct LeaseProposalCreated{

 pub   lease_proposal:Pubkey,

  pub  creator : Pubkey,

  pub lessee :Pubkey,

}



#[event]

pub struct ArbitrarVoteForLease{

 pub   lease_proposal:Pubkey,

  pub  arbitrar : Pubkey,

}


#[event]

pub struct LeaseAcceptedByLesse{

 pub   lease_proposal:Pubkey,

  pub  lesse : Pubkey,


}

#[event]

pub struct LeaseFinalize{

 pub   lease_proposal:Pubkey,

  pub  lease_status : LeaseStatus,


}


#[event]

pub struct RentPaid{

 pub   lease_proposal:Pubkey,

  


}