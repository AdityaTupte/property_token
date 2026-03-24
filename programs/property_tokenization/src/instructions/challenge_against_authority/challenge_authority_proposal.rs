use anchor_lang::prelude::*;

use crate::state::ChallengeProposal;





#[derive(Accounts)]
pub struct ChallengeAuthorityProposal<'info>{

    #[account()]
    pub proposal : Account<'info,ChallengeProposal>,


    





}