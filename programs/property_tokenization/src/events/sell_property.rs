use anchor_lang::prelude::*;


#[event]

pub struct CreateSellPropertyProposal{

    pub proposal_id : u64,
    
    pub seller : Pubkey,

    pub seller_proposal :Pubkey,

    pub property: Pubkey,

    pub amount :  u64,

}


#[event]

pub struct SubmitSnapshotForSellProposal{

        pub proposal : Pubkey,

        pub transfer_deadline_days : i64,

}
