use anchor_lang::prelude::*;

use crate::common::AuthorityType;

#[account]
pub struct SalaryPda {


    pub property_system:Pubkey,

    pub authority : AuthorityType,

    pub new_transaction_time : i64,

    pub bump:u8,


}

impl SalaryPda {
    pub const  SIZE :usize = 32 + 1 + 8 + 1 ;
}