use anchor_lang::prelude::*;

#[event]
pub struct SnapshotRequested {
    
    pub proposal_id : u64,

    pub mint: Pubkey,

    pub slot: u64,
}