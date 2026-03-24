use anchor_lang::prelude::*;

use crate::{common::{ELECT_ARBITRAR, ProposalStatus}, errors::ErrorCode, functions::submit_authority, state::ElectAuthority};


#[derive(Accounts)]
pub struct SubmitSnapshotForArbitrarAuthority<'info>{

    #[account(
        constraint = proposal.arbitrar_approvals.contains(&signer.key()) @ ErrorCode::NotAuthorized,
        constraint = proposal.is_arbitrar_approved  @ ErrorCode::AlreadyApproved
    )]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds=[
            ELECT_ARBITRAR,
            &proposal.proposal_id.to_le_bytes(),
            proposal.property_system.key().as_ref()
        ],
        bump = proposal.bump ,
        constraint = !proposal.snapshot_submitted @ ErrorCode::SnapshotAlreadySubmitted,
        constraint = proposal.status == ProposalStatus::Active @ ErrorCode::ProposalNotActive
    )]

    pub proposal : Account<'info,ElectAuthority>,

}


pub fn submit_snapshot_for_arbitrar_authority(
    ctx:Context<SubmitSnapshotForArbitrarAuthority>,

    candidate_submission_deadline: u8,

    voting_for_authority_deadline : u8,

    add_new_authority_deadline : u8,

    challenge_new_authority_deadline : u8,

    merkle_root : [u8;32],


)->Result<()>{


let proposal = &mut *ctx.accounts.proposal;

submit_authority(
    proposal,
    merkle_root,
    candidate_submission_deadline,
    voting_for_authority_deadline,
    add_new_authority_deadline,
    challenge_new_authority_deadline
)?;

Ok(())
}
