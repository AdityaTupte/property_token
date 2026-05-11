use anchor_lang::prelude::*;

use crate::{common::ProposalStatus, errors::ErrorCode, state::{ ChallengeProposal}};


#[derive(Accounts)]
pub struct AskForSnapshotOFChallengeProposal<'info>{


    #[account(
        mut, 
        constraint = signer.key() == proposed_remove_proposal.creator @ ErrorCode::UnAuthorized
    )]
    pub signer:Signer<'info>,

    #[account(
        constraint = proposed_remove_proposal.status == ProposalStatus::Draft @ ErrorCode::NotInDraft
    )]
    pub proposed_remove_proposal:Account<'info,ChallengeProposal>,

}

pub fn ask_snapshot_for_challenge_proposal(
        ctx:Context<AskForSnapshotOFChallengeProposal>
)->Result<()>{

    //emit 

    Ok(())


}