use anchor_lang::prelude::*;

use crate::common::ProposalStatus;

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