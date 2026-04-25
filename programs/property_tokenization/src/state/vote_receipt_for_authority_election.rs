use anchor_lang::prelude::*;

use crate::common::AuthorityType;




#[account]

pub struct VoteReceiptForAuthorityElection{

    pub proposal_key :Pubkey,

    pub property_system :Pubkey,

    pub authority_key : Pubkey,

    pub authority_type : AuthorityType,

}

impl VoteReceiptForAuthorityElection{

    pub const SIZE:usize =  32 +
                            32 +
                            32 +
                            1;
}