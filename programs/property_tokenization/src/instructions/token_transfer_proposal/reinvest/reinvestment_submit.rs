use anchor_lang::prelude::*;

use crate::{common::{HARDCODED_PUBKEY, ProposalStatus, USEREINVESTMENTOKEN}, errors::ErrorCode, functions::submit, state::TokenTransferProposal, };



#[derive(Accounts)]
#[instruction(property_system_account:Pubkey,proposal_id:u64)]
pub struct ReinvestSubmitSnapshot<'info>{

     #[account(mut,
    address = HARDCODED_PUBKEY  @ ErrorCode::UnAuthorized,
    )]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds=[
            USEREINVESTMENTOKEN,
            property_system_account.as_ref(),
            &proposal_id.to_le_bytes(),
        ],
        bump = proposal.bump,
        constraint = !proposal.snapshot_submitted @ ErrorCode::SnapshotAlreadySubmitted,
        constraint = proposal.status == ProposalStatus::Approved @ ErrorCode::ProposalNotApproved
    )]
    pub proposal : Account<'info,TokenTransferProposal>,

}

pub fn reinvest_submit_snapshot(
    ctx:Context<ReinvestSubmitSnapshot>,
     _property_system_account:Pubkey,_proposal_id:u64,
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