use anchor_lang::prelude::*;

use crate::{common::{HARDCODED_PUBKEY, ProposalStatus, RT_CHG_PROPOSAL_SEEDS}, errors::ErrorCode, functions::submit, state::RTChgProposal};

#[derive(Accounts)]
#[instruction(property_system_account:Pubkey,proposal_id:u64)]
pub struct SubmitSnapshot<'info>{

    #[account(mut,
    address = HARDCODED_PUBKEY  @ ErrorCode::UnAuthorized,
    )]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds=[
            RT_CHG_PROPOSAL_SEEDS,
            property_system_account.as_ref(),
            &proposal_id.to_le_bytes(),
            
        ],
        bump = proposal.bump,
        constraint = !proposal.snapshot_submitted @ ErrorCode::SnapshotAlreadySubmitted,
        constraint = proposal.status == ProposalStatus::Draft @ ErrorCode::NotInDraft
    )]

    pub proposal : Account<'info,RTChgProposal>,

}

pub fn rtc_submit_snapshot(
    ctx:Context<SubmitSnapshot>,
    _property_system_account:Pubkey,
    _proposal_id:u64,
    merkle_root : [u8;32],
    closing_days_gap : u8,
    threshold_submission_deadline_days : u8 ,
    voting_for_threshold_deadline_days : u8,
    add_new_threshold_deadline_days : u8,
    challenge_new_threshold_deadline_days : u8,
    vote_threshold :u64,
) ->Result<()>{

    require!( 0 < vote_threshold  && vote_threshold< ctx.accounts.proposal.total_voting_power, ErrorCode::InvalidVotingThreshold);

    require!(closing_days_gap <= 30 && closing_days_gap>0,ErrorCode::ClosingDay);

    require!(threshold_submission_deadline_days > 0, ErrorCode::TransferDeadline);

    let proposal = &mut *ctx.accounts.proposal;

    submit(proposal, merkle_root, closing_days_gap,vote_threshold,threshold_submission_deadline_days)?;
    
    proposal.voting_for_threshold_deadline = proposal.challenge_new_threshold_deadline
                                                            .checked_add(24*60*60*(voting_for_threshold_deadline_days as i64))
                                                            .ok_or(ErrorCode::MathOverflow)?;
    
    proposal.add_new_threshold_deadline = proposal.voting_for_threshold_deadline
                                                            .checked_add(24*60*60*(add_new_threshold_deadline_days as i64))
                                                            .ok_or(ErrorCode::MathOverflow)?;

    proposal.challenge_new_threshold_deadline = proposal.add_new_threshold_deadline
                                                            .checked_add(24*60*60*(challenge_new_threshold_deadline_days as i64))
                                                            .ok_or(ErrorCode::MathOverflow)?;

    Ok(())



}