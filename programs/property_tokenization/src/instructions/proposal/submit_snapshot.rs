use anchor_lang::prelude::*;
use crate::constant::*;
use crate::errors::ErrorCode;
use crate::state::TransferLandDetail;

#[derive(Accounts)]

pub struct SubmitSnapshot<'info>{

    #[account(
        constraint = proposal.arbitrar_approval.contains(&signer.key()),
        constraint = proposal.arbitrar_approved
    )]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds=[
            TRANSFERPROPOSAL,
            &proposal.proposal_id.to_le_bytes(),
            proposal.source_property_system.as_ref()
        ],
        bump = proposal.bump
    )]

    pub proposal : Account<'info,TransferLandDetail>,

}

pub fn submit_snapshot(
    ctx:Context<SubmitSnapshot>,
    merkle_root : [u8;32],
)->Result<()>{

    let proposal = &mut ctx.accounts.proposal;

    let current_time = Clock::get()?.unix_timestamp;

    require!(!proposal.snapshot_submitted,ErrorCode::SnapshotAlreadySubmitted);


    let one_day: i64 = 24 * 60 * 60;

    let three_days = one_day
        .checked_mul(3)
        .ok_or(ErrorCode::MathOverflow)?;

    proposal.start_time = current_time
        .checked_add(one_day)
        .ok_or(ErrorCode::MathOverflow)?;

    proposal.end_time = current_time
        .checked_add(three_days)
        .ok_or(ErrorCode::MathOverflow)?;

    proposal.merkle_root = merkle_root;

    proposal.snapshot_submitted = true;

    proposal.proposal_status = ProposalStatus::Active as u8;

    
    Ok(())


}
