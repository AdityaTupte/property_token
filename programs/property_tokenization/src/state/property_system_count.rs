use anchor_lang::prelude::*;

#[account]

pub struct PropertySystemCounter {

    pub total_property_system : u64, //total number of property system created till now

    pub bump : u8,

}