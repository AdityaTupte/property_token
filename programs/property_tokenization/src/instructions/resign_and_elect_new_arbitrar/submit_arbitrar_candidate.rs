use anchor_lang::prelude::*;

use crate::{common::{AUTHORITY_CANDIDATE, AuthorityType, CANDIDATE_PROFILE, ELECT_ARBITRAR, PROPERTY_SYSTEM_SEEDS, ProposalStatus}, errors::ErrorCode, state::{AuthorityCandidate, CandidateProfile, ElectAuthority, PropertySystemAccount}};

#[derive(Accounts)]
#[instruction(
    property_system_id : u64,
    proposal_id : u64
)]
pub struct SubmitArbitrarCandidate<'info>{

    #[account(
        mut,
    )]
    pub signer : Signer<'info>,

    #[account(
        seeds=[
            PROPERTY_SYSTEM_SEEDS,
            &property_system_id.to_le_bytes(),
        ],
        bump = property_system.bump
    )]
    pub property_system: Account<'info,PropertySystemAccount>,

    #[account(
        seeds=[
            ELECT_ARBITRAR,
            property_system.key().as_ref(),
            &proposal_id.to_le_bytes()
        ],
        bump = proposal.bump ,
        constraint = proposal.snapshot_submitted @ ErrorCode::SnapshotNotSubmitted,
        constraint = proposal.status == ProposalStatus::Active @ ErrorCode::ProposalNotActive
    )]
    pub proposal : Account<'info,ElectAuthority>,

    #[account(
        mut,
        seeds=[
                CANDIDATE_PROFILE,
                signer.key().as_ref()
        ],
        bump = candidate_profile.bump,
        // constraint = candidate_profile.is_verified  @ ErrorCode::NotVerfied,
        constraint = !candidate_profile.is_blacklisted @ ErrorCode::Blacklisted 
    )]
    pub candidate_profile : Account<'info,CandidateProfile>,

    #[account(
        init,
        payer= signer,
        seeds=[
            AUTHORITY_CANDIDATE,
            property_system.key().as_ref(), 
            proposal.key().as_ref(),
            signer.key().as_ref()       
        ],
        bump,
        space = 8 + AuthorityCandidate::SIZE
    )]
    pub new_registration : Account<'info,AuthorityCandidate>,

    pub system_program : Program<'info,System>,

}


pub fn submit_arbitrar_candidate(
    ctx:Context<SubmitArbitrarCandidate>,
    _property_system_id : u64,
    _proposal_id : u64
)->Result<()>{

    let current_time = Clock::get()?.unix_timestamp;

    let proposal = &ctx.accounts.proposal;

    require!(
        current_time < proposal.candidate_submission_deadline
        ,ErrorCode::CandidateSubmissionDeadline
    );

    let new_registration = &mut ctx.accounts.new_registration;

    let candidate_profile = &mut ctx.accounts.candidate_profile;

    new_registration.candidate = ctx.accounts.signer.key(); 

    new_registration.proposal = proposal.key();

    new_registration.property_system = ctx.accounts.property_system.key();

    new_registration.authority_type = AuthorityType::ARBITRATOR; 

    new_registration.bump = ctx.bumps.new_registration;

    candidate_profile.total_applied += 1;

    Ok(())




}


