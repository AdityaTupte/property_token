use anchor_lang::prelude::*;
use crate::common::{ProposalStatus, SELLPROPERTY};

use crate::errors::ErrorCode;
use crate::functions::submit;
use crate::state::{PropertySellProposal};

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
            SELLPROPERTY,
            proposal.property_system_account.as_ref(),
            &proposal.proposal_id.to_le_bytes(),
        ],
        bump = proposal.bump,
        constraint = !proposal.snapshot_submitted @ ErrorCode::SnapshotAlreadySubmitted,
        constraint = proposal.status == ProposalStatus::Draft @ ErrorCode::NotInDraft
    )]

    pub proposal : Account<'info,PropertySellProposal>,

}

pub fn sell_submit_snapshot(
    ctx:Context<SubmitSnapshot>,
    merkle_root : [u8;32],
    closing_days_gap : u8,
    transfer_deadline_days : u8 ,
    vote_threshold :u64,
)->Result<()>{

    require!( 0 < vote_threshold  && vote_threshold< ctx.accounts.proposal.total_voting_power, ErrorCode::InvalidVotingThreshold);

    require!(closing_days_gap <= 30 && closing_days_gap>0,ErrorCode::ClosingDay);

    require!(transfer_deadline_days > 0, ErrorCode::TransferDeadline);

    let proposal = &mut *ctx.accounts.proposal;

    submit(proposal, merkle_root, closing_days_gap,vote_threshold,transfer_deadline_days)?;
    
    Ok(())


}
