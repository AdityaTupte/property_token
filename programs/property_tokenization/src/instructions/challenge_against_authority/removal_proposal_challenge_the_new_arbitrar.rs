use anchor_lang::prelude::*;

use crate::{common::{AUTHORITY_CANDIDATE, AuthorityType, CHALLENGEAUTHORITY, PROPERTY_SYSTEM_SEEDS, ProposalStatus, REMOVEAUTHORITY}, errors::ErrorCode, functions::challenge_authority, state::{AuthorityCandidate, ChallengeProposal, ElectAuthority, PropertySystemAccount}};


#[derive(Accounts)]

pub struct RemovalProposalChallengeNewArbitrar<'info>{

    pub signer:Signer<'info>,

    #[account(
        seeds =[
            CHALLENGEAUTHORITY,
            &proposal.proposal_id.to_le_bytes(),
            property_system.key().as_ref(),
        ],
        bump = proposal.bump,
        constraint = proposal.status == ProposalStatus::Executed @ ErrorCode::ProposalNotExecuted
    )]
    pub proposal : Account<'info,ChallengeProposal>,

   #[account(
        mut,
        seeds=[
            REMOVEAUTHORITY,
            proposal.key().as_ref(),
            property_system.key().as_ref(),
        ],
        bump = removal_proposal.bump,
        constraint = removal_proposal.snapshot_submitted @ ErrorCode::SnapshotNotSubmitted,
        constraint = removal_proposal.status == ProposalStatus::Passed @ ErrorCode::ProposalNotPassed,
        constraint = removal_proposal.authority_type == AuthorityType::ARBITRATOR
    )]
    pub removal_proposal : Account<'info,ElectAuthority>,

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
            removal_proposal.key().as_ref(),
            property_system.key().as_ref()
        ],
        bump = challenge_from.bump
    )]
    pub challenge_from: Account<'info,AuthorityCandidate>,
    
    #[account(
        seeds=[
            AUTHORITY_CANDIDATE,
            challenge_from.candidate.as_ref(),
            removal_proposal.key().as_ref(),
            property_system.key().as_ref()
        ],
        bump = challenge_from.bump
    )]
    pub challenge_to: Account<'info,AuthorityCandidate>,

}


pub fn challenge_new_trustee(
    ctx:Context<RemovalProposalChallengeNewArbitrar>
)->Result<()>{

    challenge_authority(
        &mut *ctx.accounts.removal_proposal,
        &ctx.accounts.challenge_from, 
        & ctx.accounts.challenge_from.candidate, 
        &ctx.accounts.challenge_to, 
        &ctx.accounts.challenge_to.candidate,
    )?;

    Ok(())


}