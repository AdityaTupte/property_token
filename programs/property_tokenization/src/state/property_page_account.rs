use anchor_lang::prelude::*;

use crate::common::MAX_LAND_PER_PAGE;

#[account]
pub struct PropertyPage{

    pub property_system : Pubkey,

    pub page : u16,

    pub land : Vec<Pubkey>,

    pub bump : u8,

}


impl PropertyPage {
    
    pub const  SIZE: usize = 

                32 +
                2 +
                4 + (32 * MAX_LAND_PER_PAGE ) + 
                1 ;
}