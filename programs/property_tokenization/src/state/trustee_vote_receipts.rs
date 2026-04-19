use anchor_lang::prelude::*;


#[account]
pub struct TrusteeSellProposalVoteReceipts{


    pub property_system_key:Pubkey,
    pub proposal_key:Pubkey,
    pub trustee_key:Pubkey,
    pub bump:u8,

}

impl TrusteeSellProposalVoteReceipts {
    pub const SIZE: usize = 32 + 32 + 32 + 1;
}