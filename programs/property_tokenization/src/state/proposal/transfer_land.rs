use anchor_lang::prelude::*;

use crate::{constant::{Arbitrable, ProposalStatus, ProposalType, Snapshot}, state::MAX_ARBITRATOR};

#[account]

pub struct TransferLandDetail{

    pub proposal_id: u64,

    pub land : Pubkey,

    pub amount_to_transfer : u64,

    pub seller : Pubkey,

    pub seller_wallet : Pubkey,

    pub buyer : Pubkey,

    pub buyer_wallet : Pubkey, 

    pub merkle_root: [u8; 32],

    pub arbitrar_approval: Vec<Pubkey>,

    pub arbitrar_approved : bool,

    pub total_voting_power: u64,

    pub votes_for: u64,
   
    pub votes_against: u64,

    pub vote_required: u64,
   
    pub start_time: i64,
    
    pub end_time: i64,
    
    pub transfer_window:i64,
    
    pub proposal_status : ProposalStatus,

    pub snapshot_submitted : bool,

    pub proposal_type : ProposalType,

    pub bump : u8,
  
}



impl TransferLandDetail {
    pub const SIZE :usize = 
                    8 +
                    32 + 
                    8 +
                    32 +
                    32 +
                    32 +
                    32 +
                    32 +
                    4 + (32 * MAX_ARBITRATOR) +
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
                    1 ;


     pub fn initialize(
        &mut self,
        proposal_id: u64,
        land: Pubkey,
        seller: Pubkey,
        seller_wallet: Pubkey,
        buyer: Pubkey,
        buyer_wallet: Pubkey,
        amount: u64,
        proposal_type: ProposalType,
    ) {
        self.proposal_id = proposal_id;
        self.land = land;
        self.seller = seller;
        self.seller_wallet = seller_wallet;
        self.buyer = buyer;
        self.buyer_wallet = buyer_wallet;
        self.amount_to_transfer = amount;
        self.votes_for = 0;
        self.votes_against = 0;
        self.snapshot_submitted = false;
        self.arbitrar_approved = false;
        self.proposal_type = proposal_type;
    }

}


impl Arbitrable for TransferLandDetail {

    fn arbitrar_list(&mut self) -> &mut Vec<Pubkey> {
        
       return  &mut self.arbitrar_approval;
    }

    fn arbitrar_approved(&mut self) -> &mut bool {
        return  &mut self.arbitrar_approved ;
    }

    fn proposal_id(& self) -> u64 {
        return self.proposal_id;
    }
    
}

impl Snapshot for TransferLandDetail  {

    fn merkle_root(&mut self) -> &mut [u8;32] {
        return &mut self.merkle_root;
    }

    fn start_time(&mut self) -> &mut i64 {
        return &mut self.start_time;
    }
    fn end_time(&mut self) -> &mut i64 {
        return &mut self.end_time;
    }
    
    fn snapshot_submitted(&mut self) -> &mut bool {
         return &mut self.snapshot_submitted;
    }

    fn total_voting_power(& self) -> u64 {
         return self.total_voting_power;
    }

    fn vote_required(&mut self) -> &mut u64 {
         return &mut self.vote_required;
    }

    fn proposal_status(&mut self) -> &mut ProposalStatus {
         return &mut self.proposal_status;
    }


}


