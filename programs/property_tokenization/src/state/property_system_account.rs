use anchor_lang::prelude::*;

#[account]

pub struct PropertySystemAccount{

    pub property_system_id : u64,

    pub governance_mint: Pubkey,        // Governance SPL mint (voting power)

    pub treasury: Pubkey,               // PDA vault for rent / fees / liquidation

    pub trustee_registry: Pubkey,       // PDA managing trustees

    pub arbitrator_registry: Pubkey,    // PDA managing arbitrators

    pub total_properties: u64, 

    pub total_token_supply :u64,         // Global counter (land_id source)

    pub created_at: i64,                // Timestamp of system creation

    pub creator: Pubkey,                // Genesis initializer (NOT governance)

    pub ready_for_listing: bool,             // Flag indicating if system is ready for property listing

    pub bump : u8,


}

impl PropertySystemAccount{

pub const SIZE : usize = 8 + 32 + 32 + 32 + 32 + 8 + 8 + 8 + 32 + 1 +1  ;



}