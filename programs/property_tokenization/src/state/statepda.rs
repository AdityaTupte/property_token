use anchor_lang::prelude::*;

pub const MAX_STATE_NAME :usize = 64;
pub  const MAX_STATE_ACUTHORITY : usize = 10;
#[account]
pub struct State{

    pub state_id : u16,

    pub state_name: String,

    pub country_id : u64,

    pub authorities : Vec<Pubkey>,

    pub threshold : u8,

    pub bump : u8,

}

impl State {

    pub const SIZE: usize =

        8 +
        4 + MAX_STATE_NAME +
        8 +
        4  + (32 * MAX_STATE_ACUTHORITY) +
        1 + 
        1 ;
}

