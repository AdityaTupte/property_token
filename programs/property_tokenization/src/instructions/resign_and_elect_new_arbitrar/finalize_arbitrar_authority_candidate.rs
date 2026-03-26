use anchor_lang::prelude::*;

use crate::{common::{AUTHORITY_CANDIDATE, CANDIDATE_PROFILE, ELECT_ARBITRAR, PROPERTY_SYSTEM_SEEDS, ProposalStatus}, errors::ErrorCode, functions::finalize_candidate, state::{AuthorityCandidate, CandidateProfile, ElectAuthority, PropertySystemAccount}};



#[derive(Accounts)]

pub struct FinalizeArbitrarAuthorityCandiate<'info>{

    pub signer:Signer<'info>,

    #[account(
        mut,
        seeds=[
            ELECT_ARBITRAR,
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
            authority_candidate.candidate.as_ref(),
            proposal.key().as_ref(),
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


pub fn finalize_arbitrar_authority_candiate(
    ctx:Context<FinalizeArbitrarAuthorityCandiate>
)->Result<()> {

    finalize_candidate(
        &mut *ctx.accounts.proposal,
        &mut ctx.accounts.authority_candidate,
        &mut ctx.accounts.candidate_profile,        
    )?;
    

    Ok(())
}