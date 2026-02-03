use anchor_lang::prelude::*;

pub const MAX_TRUSTEES: usize = 5 ;
#[account]

pub struct TrusteeRegistry{

pub property_system_accout: Pubkey,      // Property system this trustee registry belongs to

pub trustees: [Pubkey; MAX_TRUSTEES],    // Fixed list of trustee public keys

pub total_trustees: u8,                  // Number of active trustees in the list

pub threshold: u8,                       // Minimum approvals required for a valid decision

pub bump: u8,                            // PDA bump seed for address derivation

}

impl TrusteeRegistry {
    
    pub const SIZE : usize = 32 + MAX_TRUSTEES  * 32 +  1 + 1 +1 ; 
}

