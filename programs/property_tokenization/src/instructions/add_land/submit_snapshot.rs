use anchor_lang::prelude::*;
use crate::constant::*;
use crate::errors::ErrorCode;
use crate::state::{BuyLandProposalDetail, TransferLandDetail};

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
            BUYPROPOSAL,
            proposal.buyer_property_system.as_ref(),
            &proposal.proposal_id.to_le_bytes(),
        ],
        bump = proposal.bump
    )]

    pub proposal : Account<'info,BuyLandProposalDetail>,

}

pub fn submit_snapshot_for_buyer(
    ctx:Context<SubmitSnapshot>,
    merkle_root : [u8;32],
    total_voting_power: u64,
)->Result<()>{

    let proposal = &mut ctx.accounts.proposal;

    let current_time = Clock::get()?.unix_timestamp;

    require!(!proposal.snapshot_submitted,ErrorCode::SnapshotAlreadySubmitted);


    let one_day: i64 = 24 * 60 * 60;

    let ten_days = one_day
        .checked_mul(10)
        .ok_or(ErrorCode::MathOverflow)?;

    proposal.start_time = current_time
        .checked_add(one_day)
        .ok_or(ErrorCode::MathOverflow)?;

    proposal.end_time = current_time
        .checked_add(ten_days)
        .ok_or(ErrorCode::MathOverflow)?;

    proposal.merkle_root = merkle_root;

    proposal.snapshot_submitted = true;

    proposal.total_voting_power = total_voting_power;

    let vote_required = total_voting_power
                                        .checked_mul(65)
                                        .ok_or(ErrorCode::MathOverflow)?
                                        .checked_add(99)
                                        .ok_or(ErrorCode::MathOverflow)?
                                        / 100; 
    
    proposal.vote_required = vote_required;

    proposal.proposal_status = ProposalStatus::Active as u8;

    
    Ok(())


}
