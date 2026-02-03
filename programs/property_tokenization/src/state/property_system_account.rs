use anchor_lang::prelude::*;

#[account]

pub struct PropertySystemAccount{

    
    pub governance_mint: Pubkey,        // Governance SPL mint (voting power)

    pub treasury: Pubkey,               // PDA vault for rent / fees / liquidation

    pub trustee_registry: Pubkey,       // PDA managing trustees

    pub arbitrator_registry: Pubkey,    // PDA managing arbitrators

    pub total_properties: u64,          // Global counter (land_id source)

    pub max_page: u16,                  // Page size for indexing (pagination)

    pub created_at: i64,                // Timestamp of system creation

    pub creator: Pubkey,                // Genesis initializer (NOT governance)

}

impl PropertySystemAccount{

pub const SIZE : usize = 32 + 32 + 32 + 32 + 8 + 2 + 8 + 32 ;



}