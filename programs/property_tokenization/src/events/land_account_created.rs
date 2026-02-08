use anchor_lang::prelude::*;


#[event]

pub struct LandAccountCreated{

    pub land_account : Pubkey,

    pub state : Pubkey,
    
    pub issued_by : Pubkey,

    pub land_metadata: Pubkey,



}