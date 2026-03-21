use anchor_lang::prelude::*;

use crate::{common::{AUTHORITY_CANDIDATE, ELECT_TRUSTEE, PROPERTY_SYSTEM_SEEDS, ProposalStatus}, errors::ErrorCode, functions::challenge_authority, state::{AuthorityCandidate, ElectAuthority, PropertySystemAccount}};


#[derive(Accounts)]

pub struct ChallengeNewTrustee<'info>{

    
    pub signer:Signer<'info>,

    #[account(
        mut,
        seeds=[
            ELECT_TRUSTEE,
            &proposal.proposal_id.to_le_bytes(),
            property_system.key().as_ref(),
        ],
        bump = proposal.bump,
        constraint = proposal.snapshot_submitted @ ErrorCode::SnapshotNotSubmitted,
        constraint = proposal.status == ProposalStatus::Passed @ ErrorCode::ProposalNotPassed
    )]
    pub proposal : Account<'info,ElectAuthority>,

    #[account(
        seeds=[
            PROPERTY_SYSTEM_SEEDS,
            &property_system.property_system_id.to_le_bytes(),
        ],
        bump = property_system.bump
    )]
    pub property_system:Account<'info,PropertySystemAccount>,

    #[account(
        seeds=[
            AUTHORITY_CANDIDATE,
            challenge_from.candidate.as_ref(),
            proposal.key().as_ref(),
            property_system.key().as_ref()
        ],
        bump = challenge_from.bump
    )]
    pub challenge_from: Account<'info,AuthorityCandidate>,
    
    #[account(
        seeds=[
            AUTHORITY_CANDIDATE,
            challenge_from.candidate.as_ref(),
            proposal.key().as_ref(),
            property_system.key().as_ref()
        ],
        bump = challenge_from.bump
    )]
    pub challenge_to: Account<'info,AuthorityCandidate>,

}


pub fn challenge_new_trustee(
    ctx:Context<ChallengeNewTrustee>
)->Result<()>{

    challenge_authority(
        &mut *ctx.accounts.proposal,
        &ctx.accounts.challenge_from, 
        & ctx.accounts.challenge_from.key(), 
        &ctx.accounts.challenge_to, 
        &ctx.accounts.challenge_to.key(),
    )?;

    Ok(())


}