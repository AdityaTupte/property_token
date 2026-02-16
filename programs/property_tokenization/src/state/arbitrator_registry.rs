use anchor_lang::prelude::*;

pub const MAX_ARBITRATOR: usize = 5 ;
#[account]

pub struct ArbitratorRegistry{

pub property_system_account: Pubkey,        // Property system this arbitrator registry belongs to

pub arbitrator: [Pubkey; MAX_ARBITRATOR],  // Fixed list of arbitrator public keys

pub total_arbitrator: u8,                  // Number of active arbitrators

pub threshold: u8,                         // Minimum approvals required for arbitration decisions

pub bump: u8,                              // PDA bump seed for address derivation


}

impl ArbitratorRegistry {
    
    pub const SIZE : usize = 32 + MAX_ARBITRATOR  * 32 +  1 + 1 +1 ; 
}

