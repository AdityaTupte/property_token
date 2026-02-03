use anchor_lang::prelude::*;

#[account]

pub struct PropertySystemCounter {

    pub total_property_system : u64,

    pub bump : u8,

}