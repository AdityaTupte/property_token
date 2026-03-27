use anchor_lang::prelude::*;

use crate::{common::{AUTHORITY_CANDIDATE, AuthorityType, CANDIDATE_PROFILE, CHALLENGEAUTHORITY, PROPERTY_SYSTEM_SEEDS, ProposalStatus, REMOVEAUTHORITY}, errors::ErrorCode, functions::finalize_candidate, state::{AuthorityCandidate, CandidateProfile, ChallengeProposal, ElectAuthority, PropertySystemAccount}};



#[derive(Accounts)]

pub struct RmFinalizeArbitrarCandidate<'info>{

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
            authority_candidate.candidate.as_ref(),
            removal_proposal.key().as_ref(),
            property_system.key().as_ref()
        ],
        bump = authority_candidate.bump
    )]
    pub authority_candidate: Account<'info,AuthorityCandidate>,

    #[account(
        seeds=[
                CANDIDATE_PROFILE,
                signer.key().as_ref()
        ],
        bump = candidate_profile.bump,
        constraint = candidate_profile.is_verified  @ ErrorCode::NotVerfied,
        constraint = !candidate_profile.is_blacklisted @ ErrorCode::Blacklisted 
    )]
    pub candidate_profile : Account<'info,CandidateProfile>,

}


pub fn finalize_trustee_authority_candiate(
    ctx:Context<RmFinalizeArbitrarCandidate>
)->Result<()> {

    finalize_candidate(
        &mut *ctx.accounts.removal_proposal,
        &mut ctx.accounts.authority_candidate,
        &mut ctx.accounts.candidate_profile,        
    )?;
    

    Ok(())
}