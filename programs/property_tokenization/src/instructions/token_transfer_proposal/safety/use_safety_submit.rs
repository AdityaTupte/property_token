use anchor_lang::prelude::*;

use crate::{constant::{ProposalStatus, SAFETYPROPOSAL}, errors::ErrorCode, functions::submit, state::SafetyProposal};



#[derive(Accounts)]
pub struct SubmitSnapshot<'info>{

    #[account(
        constraint = proposal.arbitrar_approvals.contains(&signer.key()),
        constraint = proposal.is_arbitrar_approved
    )]
    pub signer : Signer<'info>,

    #[account(
        mut,
        seeds=[
            SAFETYPROPOSAL,
            proposal.property_system.as_ref(),
            &proposal.proposal_id.to_le_bytes(),
        ],
        bump = proposal.bump,
        constraint = !proposal.snapshot_submitted @ ErrorCode::SnapshotAlreadySubmitted,
        constraint = proposal.status == ProposalStatus::Draft @ ErrorCode::NotInDraft
    )]
    pub proposal : Account<'info,SafetyProposal>,

}

pub fn saftey_submit_snapshot(
    ctx:Context<SubmitSnapshot>,
    merkle_root : [u8;32],
    closing_days_gap : u8,
    deadline_days : u8 ,
    vote_threshold :u64,
)->Result<()>{

    require!(vote_threshold < ctx.accounts.proposal.total_voting_power, ErrorCode::InvalidVotingThreshold);

    require!(closing_days_gap>0,ErrorCode::ClosingDay);

    let proposal = &mut *ctx.accounts.proposal;

    submit(proposal, merkle_root, closing_days_gap,vote_threshold,deadline_days)?;
    
    Ok(())


}