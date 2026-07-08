use anchor_lang::prelude::*;

use crate::{common::ProposalStatus, errors::ErrorCode, events::SnapshotRequested, state::ChallengeProposal};


#[derive(Accounts)]
pub struct AskForSnapshotOFChallengeProposal<'info>{


    #[account(
        mut, 
        constraint = signer.key() == challenge_proposal.creator @ ErrorCode::UnAuthorized
    )]
    pub signer:Signer<'info>,

    #[account(
        constraint = challenge_proposal.status == ProposalStatus::Draft @ ErrorCode::NotInDraft
    )]
    pub challenge_proposal:Account<'info,ChallengeProposal>,

}

pub fn ask_snapshot_for_challenge_proposal(
        ctx:Context<AskForSnapshotOFChallengeProposal>
)->Result<()>{

    emit!(
        SnapshotRequested{
            proposal_id:ctx.accounts.challenge_proposal.proposal_id,
            proposal_key:ctx.accounts.challenge_proposal.key(),
            mint:ctx.accounts.challenge_proposal.key(),
            slot:Clock::get()?.slot,
        }
    );
    

    Ok(())


}