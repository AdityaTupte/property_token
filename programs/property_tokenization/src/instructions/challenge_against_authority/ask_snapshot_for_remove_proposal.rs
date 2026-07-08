use anchor_lang::prelude::*;

use crate::{common::ProposalStatus, errors::ErrorCode, events::SnapshotRequested, state::ProposeRemoveProposal};


#[derive(Accounts)]
pub struct AskForSnapshotOFRemoveProposal<'info>{


    #[account()]
    pub signer:Signer<'info>,

    #[account(
        constraint = proposed_remove_proposal.status == ProposalStatus::Draft @ ErrorCode::NotInDraft
    )]
    pub proposed_remove_proposal:Account<'info,ProposeRemoveProposal>

}

pub fn ask_snapshot_for_remove_proposal(
        ctx:Context<AskForSnapshotOFRemoveProposal>
)->Result<()>{

    
    emit!(
        SnapshotRequested{
            proposal_id:00,
            proposal_key:ctx.accounts.proposed_remove_proposal.key(),
            mint:ctx.accounts.proposed_remove_proposal.key(),
            slot:Clock::get()?.slot,
        }
    );

    Ok(())


}