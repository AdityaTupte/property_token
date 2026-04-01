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

impl LeaseProperty  {
    pub const SIZE : usize = 32 + 8 + 32+ 32 + 1+ 1 +8 + 8 +8 + 8  +8 +32 +8 +8 +32 +1 ;
}
