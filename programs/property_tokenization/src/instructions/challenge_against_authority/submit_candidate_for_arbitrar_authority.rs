use anchor_lang::prelude::*;

use crate::{common::{AUTHORITY_CANDIDATE, CANDIDATE_PROFILE,  PROPERTY_SYSTEM_SEEDS, ProposalStatus, REMOVEARBITRARAUTHORITYPROPOSAL,}, errors::ErrorCode, state::{AuthorityCandidate, CandidateProfile,  ElectAuthority, PropertySystemAccount}};

#[derive(Accounts)]
#[instruction(proposal_key:Pubkey,property_system_id:u64)]
pub struct  SubmitCandidateForArbitrarAuthority<'info>{
    
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

    //  #[account(
    //     seeds =[
    //         CHALLENGEAUTHORITY,
    //         &proposal.proposal_id.to_le_bytes(),
    //         property_system.key().as_ref(),
    //     ],
    //     bump = proposal.bump,
    //     constraint = proposal.status == ProposalStatus::Executed @ ErrorCode::ProposalNotExecuted
    // )]
    // pub proposal : Account<'info,ChallengeProposal>,


    #[account(
        seeds=[
            REMOVEARBITRARAUTHORITYPROPOSAL,
            property_system.key().as_ref(),
            proposal_key.as_ref()
        ],
        bump=removal_proposal.bump,
        constraint = removal_proposal.snapshot_submitted @ ErrorCode::SnapshotNotSubmitted,
        constraint = removal_proposal.status == ProposalStatus::Active @ ErrorCode::ProposalNotPassed

    )]
    pub removal_proposal: Account<'info,ElectAuthority>,

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
            removal_proposal.key().as_ref(),
            signer.key().as_ref()        
        ],
        bump,
        space = 8 + AuthorityCandidate::SIZE
    )]
    pub new_registration : Account<'info,AuthorityCandidate>,

    pub system_program : Program<'info,System>,

}

pub fn submit_candidate_for_arbitrar_authority(
    ctx:Context<SubmitCandidateForArbitrarAuthority>,
    _proposal_key:Pubkey,
    _property_system_id:u64
)->Result<()>{

    
    let current_time = Clock::get()?.unix_timestamp;

    let removal_proposal = &ctx.accounts.removal_proposal;

     require!(
        current_time < removal_proposal.candidate_submission_deadline
        ,ErrorCode::CandidateSubmissionDeadline
    );

    let new_registration = &mut ctx.accounts.new_registration;

    let candidate_profile = &mut ctx.accounts.candidate_profile;

    new_registration.candidate = ctx.accounts.signer.key(); 

    new_registration.proposal = removal_proposal.key();

    new_registration.property_system = ctx.accounts.property_system.key();

    new_registration.authority_type = ctx.accounts.removal_proposal.authority_type; 

    new_registration.bump = ctx.bumps.new_registration;

    candidate_profile.total_applied += 1;

    Ok(())

}

