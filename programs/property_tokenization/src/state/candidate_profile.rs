use anchor_lang::prelude::*;

use crate::common::ReasonType;

#[account()]

pub struct CandidateProfile{

    pub candidate:Pubkey,

    pub is_verfied : bool,

    pub total_applied : u16,

    pub total_selected_as_trustee :u16,

    pub total_selected_as_arbitrar :u16,

    pub is_blacklisted:bool,

    pub removal_count : u16,

    pub removal_reason_escalation_manner : ReasonType,

    pub metadata_hash: [u8; 32],

    pub bump : u8,

}

impl CandidateProfile {
    pub const SIZE:usize =  32 +
                            1 +
                            2 +
                            2 +
                            2 +
                            1 +
                            2 +
                            1 +
                            32 +
                            1 ;
}