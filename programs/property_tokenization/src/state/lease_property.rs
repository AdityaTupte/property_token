use anchor_lang::prelude::*;

use crate::common::{LeaseStatus};

#[account]

pub struct  LeaseProperty{


    pub property_system : Pubkey,

    pub lease_id : u64,

    pub property : Pubkey,

    pub lessee : Pubkey,

    pub status : LeaseStatus,

    pub rent_amount : u64,

    pub security_deposit : u64,

    pub periodic_pay : i64,

    pub next_payment : i64,

    pub late_payment_fee_per_day : u64,

    pub last_payement : i64,

    pub agreemenbt_hash : [u8;32],

    pub lease_start_time : i64,

    pub lease_end_time : i64,
    
    pub neutral : Pubkey,

    pub bump : u8,

}

impl LeaseProperty {
    pub const SIZE: usize =
        32 + // property_system
        8  + // lease_id
        32 + // property
        32 + // lessee
        1  + // status
        8  + // rent_amount
        8  + // security_deposit
        8  + // periodic_pay
        8  + // next_payment
        8  + // late_payment_fee_per_day
        8  + // last_payment
        32 + // agreement_hash
        8  + // lease_start_time
        8  + // lease_end_time
        32 + // neutral
        1;   // bump
}
