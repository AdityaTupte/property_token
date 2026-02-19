use anchor_lang::prelude::*;

use crate::state::{MAX_ARBITRATOR};

#[account]

pub struct TransferLandDetail{

    pub proposal_id: u64,

    pub land : Pubkey,

    pub amount_to_transfer : u64,

    pub source_property_system : Pubkey,

    pub source_treasury : Pubkey,

    pub destination_property_system : Pubkey,

    pub destination_treasury : Pubkey, 

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
    
    pub proposal_status : u8,

    pub snapshot_submitted : bool,

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
                    1 ;
}



