use anchor_lang::prelude::*;

use crate::{common::ProposalStatus, errors::ErrorCode, state::{ ProposeRemoveProposal}};


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

    //emit 

    Ok(())


}