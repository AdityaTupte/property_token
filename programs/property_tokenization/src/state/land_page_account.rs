use anchor_lang::prelude::*;
use crate::constant::*;
#[account]
pub struct LandPage{

    pub property_system : Pubkey,

    pub page : u16,

    pub land : Vec<Pubkey>,

    pub bump : u8,

}


impl LandPage {
    
    pub const  SIZE: usize = 

                32 +
                2 +
                4 + (32 * MAX_LAND_PER_PAGE ) + 
                1 ;
}