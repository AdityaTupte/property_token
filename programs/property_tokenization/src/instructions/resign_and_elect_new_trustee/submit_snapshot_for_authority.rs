use anchor_lang::prelude::*;

use crate::{common::{ELECT_TRUSTEE, HARDCODED_PUBKEY, ProposalStatus}, errors::ErrorCode, functions::submit_authority, state::ElectAuthority};


#[derive(Accounts)]
#[instruction(proposal_id:u64,property_system:Pubkey)]
pub struct SubmitSnapshotForAuthority<'info>{

    #[account(
        // constraint = proposal.arbitrar_approvals.contains(&signer.key()) @ ErrorCode::NotAuthorized,
        address=HARDCODED_PUBKEY
    )]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds=[
            ELECT_TRUSTEE,
            property_system.as_ref(),
            &proposal_id.to_le_bytes()
        ],
        bump = proposal.bump ,
        constraint = !proposal.snapshot_submitted @ ErrorCode::SnapshotAlreadySubmitted,
        constraint = proposal.status == ProposalStatus::Approved @ ErrorCode::ProposalNotApproved
    )]

    pub proposal : Account<'info,ElectAuthority>,

}


pub fn submit_snapshot_for_authority(
    ctx:Context<SubmitSnapshotForAuthority>,

    _proposal_id:u64,

    _property_system:Pubkey,

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
