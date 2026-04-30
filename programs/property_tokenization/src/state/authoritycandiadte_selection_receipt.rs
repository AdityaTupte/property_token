use anchor_lang::prelude::*;


#[account]
pub struct AuthorityCandidateSelectionRecipt{


    pub candidate_key:Pubkey,

    pub proposal:Pubkey,
    pub bump : u8,

}


impl AuthorityCandidateSelectionRecipt {
    pub const SIZE:usize= 32 + 32 + 1;
}