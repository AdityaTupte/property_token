use anchor_lang::prelude::*;

use crate::common::ProposalType;


#[account]
pub struct ArbitratorVoteReceipts{


    pub property_system_key:Pubkey,
    pub proposal_key:Pubkey,
    pub arbitrator_key:Pubkey,
    pub proposal_type : ProposalType,
    pub bump:u8,

}

impl ArbitratorVoteReceipts {
    pub const SIZE: usize = 32 + 32 + 32 + 1 + 1;
}