use anchor_lang::prelude::*;

#[account]

pub struct NEWTHRESHOLDPROPOSAL{

    pub property_system : Pubkey,

    pub new_threshold: Pubkey,

    pub proposal : Pubkey,

    pub signer : Pubkey,

    pub vote_gained : u64,

    pub new_trustee_salary_threshold : u8,

    pub new_arbitrator_salary_threshold : u8,

    pub new_dividend_threshold: u8,

    pub new_reinvestment_threshold : u8,

    pub new_safety_threshold : u8,

    pub bump : u8,
}


impl NEWTHRESHOLDPROPOSAL {
    pub const SIZE: usize =
                        32 +
                        32 +
                        32 +
                        32 +
                        8 +
                        1 +
                        1 +
                        1 +
                        1 +
                        1 +
                        1 ;   
}