use anchor_lang::prelude::*;

use crate::{common::{ CANDIDATE_PROFILE, CHALLENGEAUTHORITY, PROPERTY_SYSTEM_SEEDS, ProposalStatus, REMOVEAUTHORITY}, errors::ErrorCode, state::{ CandidateProfile, ChallengeProposal, ElectAuthority, PropertySystemAccount}};




#[derive(Accounts)]
pub struct FinalizeRemovedAuthority<'info>{

    pub signer: Signer<'info>,

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
        constraint = removal_proposal.status == ProposalStatus::Executed @ ErrorCode::ProposalNotExecuted,
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
                CANDIDATE_PROFILE,
                candidate_profile.candidate.as_ref()
        ],
        bump = candidate_profile.bump,
        constraint = candidate_profile.is_verified  @ ErrorCode::NotVerfied,
        constraint = !candidate_profile.is_blacklisted @ ErrorCode::Blacklisted 
    )]
    pub candidate_profile : Account<'info,CandidateProfile>,

}

pub fn finalize_removed_authority(
    ctx:Context<FinalizeRemovedAuthority>
)->Result<()>{

    let removal_proposal = &mut ctx.accounts.removal_proposal;

    let candidate = &mut ctx.accounts.candidate_profile;

    let index= removal_proposal.index_for_removal as usize;

    require!(index < removal_proposal.authority_to_resign.len() ,ErrorCode::AuhtorityLimitReached);

    require!(removal_proposal.authority_to_resign[index] == candidate.candidate, ErrorCode::AuthotityTypeNotMatched);

    candidate.removal_count += 1; 

    removal_proposal.index_for_removal += 1;

    Ok(())

    


}