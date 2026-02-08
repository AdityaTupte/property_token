use anchor_lang::prelude::*;


#[account]
pub struct State{

    pub state_id : u64,

    pub country_id : u64,

    pub authorities : Vec<Pubkey>,

    pub bump : u8,


}

