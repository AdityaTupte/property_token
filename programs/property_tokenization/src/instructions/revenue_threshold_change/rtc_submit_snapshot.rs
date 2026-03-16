use anchor_lang::prelude::*;

use crate::{common::{ProposalStatus, RT_CHG_PROPOSAL_SEEDS}, errors::ErrorCode, functions::submit, state::RTChgProposal};

#[derive(Accounts)]

pub struct SubmitSnapshot<'info>{

    #[account(
        constraint = proposal.arbitrar_approvals.contains(&signer.key()) @ ErrorCode::NotAuthorized,
        constraint = proposal.is_arbitrar_approved @ ErrorCode::ProposalNotApproved
    )]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds=[
            RT_CHG_PROPOSAL_SEEDS,
            &proposal.proposal_id.to_le_bytes(),
            proposal.property_system.key().as_ref()
        ],
        bump = proposal.bump,
        constraint = !proposal.snapshot_submitted @ ErrorCode::SnapshotAlreadySubmitted,
        constraint = proposal.status == ProposalStatus::Draft @ ErrorCode::NotInDraft
    )]

    pub proposal : Account<'info,RTChgProposal>,

}

pub fn rtc_submit_snapshot(
    ctx:Context<SubmitSnapshot>,
    merkle_root : [u8;32],
    closing_days_gap : u8,
    threshold_submission_deadline : u8 ,
    vote_threshold :u64,
) ->Result<()>{

    require!( 0 < vote_threshold  && vote_threshold< ctx.accounts.proposal.total_voting_power, ErrorCode::InvalidVotingThreshold);

    require!(closing_days_gap <= 30 && closing_days_gap>0,ErrorCode::ClosingDay);

    require!(threshold_submission_deadline > 0, ErrorCode::TransferDeadline);

    let proposal = &mut *ctx.accounts.proposal;

    submit(proposal, merkle_root, closing_days_gap,vote_threshold,threshold_submission_deadline)?;
    
    Ok(())



}