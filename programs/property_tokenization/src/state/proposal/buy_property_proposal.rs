use anchor_lang::prelude::*;
use crate::{constant::*, state::MAX_ARBITRATOR};

#[account()]

pub struct PropertyBuyProposal{

    pub proposal_id : u64,

    pub buyer : Pubkey,

    pub buyer_wallet : Pubkey,

    pub sell_proposal : Pubkey,

    pub property : Pubkey,

    pub sale_price : u64,

    pub merkle_root: [u8; 32],

    pub arbitrar_approvals: Vec<Pubkey>,

    pub is_arbitrar_approved : bool,

    pub total_voting_power: u64,

    pub votes_for: u64,
   
    pub votes_against: u64,

    pub vote_threshold: u64,
   
    pub start_time: i64,

    pub end_time: i64,
    
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
                            4 + (32 * MAX_ARBITRATOR) +
                            1 +
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


