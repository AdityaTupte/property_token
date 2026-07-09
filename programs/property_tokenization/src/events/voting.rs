use anchor_lang::prelude::*;



#[event]

pub struct VoteForProposal {


    pub proposal : Pubkey,

    pub voter : Pubkey,

    pub for_against : bool


}




#[event]

pub struct VoteForAuthority {


    pub proposal : Pubkey,

    pub voter : Pubkey,

    pub candidate : Pubkey,


}