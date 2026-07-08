use anchor_lang::prelude::*;


#[event]

pub struct CreateBuyPropertyProposal{

    pub proposal_id : u64,

    pub buyer : Pubkey,

    pub buyer_proposal : Pubkey,
    
    pub seller : Pubkey,

    pub seller_proposal :Pubkey,

    pub property: Pubkey,

    pub amount :  u64,

}


#[event]
pub struct BuyPropertyProposalArbitrarVote {

    pub proposal_key : Pubkey,

    pub property_system : Pubkey,

    pub voter : Pubkey,

}


#[event]
pub struct BuyPropertyProposalSnapshotRequest {

    pub proposal_key : Pubkey,

    pub property_system : Pubkey,

    pub voter : Pubkey,

}


#[event]
pub struct BuyPropertyProposalExecuted{

    pub proposal_id : u64,

    pub buyer : Pubkey,
    
    pub buyer_proposal : Pubkey,

    pub seller : Pubkey,

    pub seller_proposal :Pubkey,

    pub seller_ata : Pubkey,

    pub property: Pubkey,

    pub amount :  u64,


}