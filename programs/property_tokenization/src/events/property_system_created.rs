use anchor_lang::prelude::*;

use crate::common::AuthorityType;


#[event]
pub struct PropertySystemCreated {
    pub property_system: Pubkey,
    pub creator: Pubkey,
    pub governance_mint: Pubkey,

    // PDAs
    pub treasury: Pubkey,
    pub reinvestment: Pubkey,
    pub safety: Pubkey,
    pub dividend: Pubkey,

    // thresholds
    pub safety_threshold: u8,
    pub trustee_salary_threshold: u8,
    pub arbitrator_salary_threshold: u8,
    pub dividend_threshold: u8,
    pub reinvestment_threshold: u8,

    pub created_at: i64,
}

#[event]
pub struct AuthorityAddedForPropertySystem{
    pub property_system:Pubkey,
    pub authority:Pubkey,
    pub authority_type:AuthorityType,

}