use anchor_lang::prelude::*;


pub const MAX_STATE_NAME :usize = 64;
pub  const MAX_STATE_ACUTHORITY : usize = 10;
#[account]
pub struct State{

    pub state_id : u16,

    pub state_name: [u8;32],

    pub country_id : u16,

    pub country_pubkey : Pubkey,

    pub total_authorities : u8,

    pub threshold : u8,

    pub bump : u8,

}

impl State {

    pub const SIZE: usize =

        2 +
        32+
        2 +
        32 +
        1+
        1 + 
        1 ;
}


#[account]
pub struct StateAuthority{

    pub state_pubkey : Pubkey,

    pub bump : u8,
}

impl StateAuthority {

   pub const SIZE : usize =  32 +1 ;
}

