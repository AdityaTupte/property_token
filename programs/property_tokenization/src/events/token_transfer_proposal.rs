use anchor_lang::prelude::*;

use crate::common::ProposalType;


#[event]
pub struct CreateProposalForTokenTransfer{
    
    pub proposal : Pubkey,

    pub property_system :Pubkey,

     pub amount : u64,

    pub proposal_type  : ProposalType,

}

#[event]
pub struct SubmitForTokenTransfer{
    
    pub proposal : Pubkey,

   pub end_time : i64,

    pub proposal_type  : ProposalType,

}

#[event]
pub struct TokenTransferExecuted{
    
    pub proposal : Pubkey,

    pub property_system : Pubkey,

    pub credited_account :Pubkey,

    pub amount : u64,

    pub proposal_type  : ProposalType,

}