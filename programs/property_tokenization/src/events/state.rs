use anchor_lang::prelude::*;

#[event]

pub struct StateCreated{

    pub proposal:Pubkey,

    pub country : Pubkey,

    pub creator : Pubkey,


}


#[event]

pub struct VoteForStateProposal{

    pub proposal:Pubkey,


    pub authority : Pubkey,
    
}


#[event]

pub struct ExecuteStateProposal{

    pub proposal:Pubkey,

    pub state : Pubkey,
   
    
}


#[event]

pub struct AddedStateAuthority{


    pub state : Pubkey,

    pub authority:Pubkey,
   
    
}