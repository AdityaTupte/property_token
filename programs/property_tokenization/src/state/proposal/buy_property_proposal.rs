use anchor_lang::prelude::*;
use crate::{common::{ ProposalStatus, ProposalType}, constant::*,};

#[account]

pub struct PropertyBuyProposal{

    pub proposal_id : u64,

    pub buyer : Pubkey,

    pub buyer_wallet : Pubkey,

    pub sell_proposal : Pubkey,

    pub property : Pubkey,

    pub sale_price : u64,

    pub merkle_root: [u8; 32],

    pub arbitrar_approvals_count: u8,

    pub is_arbitrar_approved : bool,

    pub total_voting_power: u64,

    pub votes_for: u64,
   
    pub votes_against: u64,

    pub vote_threshold: u64,
   
    pub start_time: i64,

    pub end_time: i64,

    pub payment_deadline : i64,
    
    pub status : ProposalStatus,

    pub snapshot_submitted : bool,

    pub proposal_type : ProposalType,

    pub slot : u64,

    pub bump : u8,

}

impl PropertyBuyProposal {
    pub const SIZE:usize = 8 +
                            32 +
                            32 +
                            32 +
                            32 +
                            8 +
                            32 +
                            2 +
                            1 +
                            8 +
                            8 +
                            8 +
                            8 +
                            8 +
                            8 +
                            8 +
                            1 +
                            1 +
                            1 +
                            8 +
                            1 ; 

    pub fn initialize(

    &mut self,
    proposal_id: u64,
    buyer:Pubkey,
    buyer_wallet: Pubkey,
    property: Pubkey,
    seller_proposal:Pubkey,
    sale_price:u64,
    bump:u8,
    total_voting_power:u64,
){
    self.proposal_id = proposal_id;
    self.buyer = buyer;
    self.buyer_wallet = buyer_wallet;
    self.property = property;
    self.sell_proposal = seller_proposal;
    self.sale_price = sale_price;
    self.bump = bump;
    self.total_voting_power = total_voting_power;
    self.status = ProposalStatus::Draft;
    self.proposal_type = ProposalType::BUYPROPERTY;




}                        

}

impl BaseProposal for PropertyBuyProposal {

    fn proposal_id(&mut self) -> &mut u64 {
        &mut self.proposal_id
    }

    fn merkle_root(&mut self) -> &mut [u8; 32] {
        &mut self.merkle_root
    }

     fn arbitrar_total_count(&mut self) -> &mut u8 {
        &mut self.arbitrar_approvals_count 
    }


    // fn arbitrar_list(&mut self) -> &mut Vec<Pubkey> {
    //     &mut self.arbitrar_approvals
    // }

    fn arbitrar_approved(&mut self) -> &mut bool {
        &mut self.is_arbitrar_approved
    }

    fn proposal_status(&mut self) -> &mut ProposalStatus {
        &mut self.status
    }

    fn snapshot_submitted(&mut self) -> &mut bool {
        &mut self.snapshot_submitted
    }

    fn slot(&mut self) -> &mut u64 {
        &mut self.slot
    }
    
    fn bump(&mut self) -> &mut u8 {
        &mut self.bump
    }
}


impl Governance for PropertyBuyProposal {

    fn start_time(&mut self) -> &mut i64 {
        &mut self.start_time
    }

    fn end_time(&mut self) -> &mut i64 {
        &mut self.end_time
    }

   
    fn total_voting_power(&mut self) -> &mut u64 {
        &mut self.total_voting_power
    }

    fn vote_threshold(&mut self) -> &mut u64 {
        &mut self.vote_threshold
    }

    fn votes_for(&mut self) -> &mut u64 {
        &mut self.votes_for
    }

    fn votes_against(&mut self) -> &mut u64 {
        &mut self.votes_against
    }

    fn proposal_type(&mut self) -> &mut ProposalType {
        &mut self.proposal_type
    }

    fn deadline(&mut self) -> &mut i64 {
        &mut self.payment_deadline
    }
}



// impl Governance for PropertyBuyProposal {

//     fn proposal_id(&mut self) ->&mut u64 {
//         return &mut self.proposal_id;
//     }

//     fn start_time(&mut self) -> &mut i64 {
//         return &mut self.start_time;
//     }

//     fn end_time(&mut self) -> &mut i64 {
//         return &mut self.end_time;
//     }
//     fn merkle_root(&mut self) -> &mut [u8;32] {
//         return &mut self.merkle_root;
//     }

//     fn arbitrar_list(&mut self) -> &mut Vec<Pubkey> {
//         return  &mut self.arbitrar_approvals;
//     }
    
//     fn arbitrar_approved(&mut self)-> &mut bool {
//         return  &mut self.is_arbitrar_approved ;
//     }

//     fn total_voting_power(&mut self) -> &mut u64 {
//         return &mut self.total_voting_power;
//     }
//     fn vote_threshold(&mut self) -> &mut u64 {
//         return &mut self.vote_threshold;
//     }

//     fn votes_for(&mut self) -> &mut u64 {
//         return &mut self.votes_for;
//     }

//     fn votes_against(&mut self) -> &mut u64 {
//         return &mut self.votes_against;
//     }

//     fn proposal_status(&mut self) -> &mut ProposalStatus {
//         return &mut self.status;
//     }

//     fn snapshot_submitted(&mut self) -> &mut bool {
//         return &mut self.snapshot_submitted;
//     }

//     fn proposal_type(&mut self) -> &mut ProposalType {
//         return &mut self.proposal_type;
//     }

//     fn slot(&mut self) -> &mut u64 {
//         return &mut self.slot;
//     }

//     fn deadline(&mut self) -> &mut i64 {
//         return &mut self.payment_deadline;
//     }

//     fn bump(&mut self) -> &mut u8 {
//         return &mut self.bump;
//     }

// }

// impl buyer_transfer for PropertyBuyProposal {
    
//     fn owner (&mut self) -> &mut Pubkey {
//         return &mut self.buyer;
//     }

//     fn owner_wallet(&mut self) -> &mut Pubkey {
//         return &mut self.buyer_wallet;
//     }
//     fn property_account(&mut self) -> &mut Pubkey {
//         return &mut self.property;
//     }

//     fn purchase_price(&mut self) -> &mut u64 {
//         return &mut self.sale_price;
//     }

//     fn seller_proposal (&mut self) -> &mut Pubkey {
//         return &mut self.sell_proposal;
//     }
// }






