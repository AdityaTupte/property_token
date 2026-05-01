use anchor_lang::prelude::*;

use crate::{common::{ARBITRAR_RECEIPT_SEEDS, ARBITRAR_REGISTRYSEEDS, AUTHORITY_CANDIDATE, CANDIDATE_PROFILE, ELECT_ARBITRAR, PROPERTY_SYSTEM_SEEDS, ProposalStatus}, errors::ErrorCode, functions::finalize_candidate, state::{ArbitratorRecepit, ArbitratorRegistry, AuthorityCandidate, CandidateProfile, ElectAuthority, PropertySystemAccount}};



#[derive(Accounts)]
#[instruction(candidate_pubkey:Pubkey,proposal_id:u64,property_system_id:u64)]
pub struct FinalizeArbitrarAuthorityCandiate<'info>{

    #[account(mut)]
    pub signer:Signer<'info>,

    #[account(
        constraint = candidate.key() == candidate_pubkey
    )]
    pub candidate: SystemAccount<'info>,
    

    #[account(
        init,
        payer = signer,
        seeds = [
            ARBITRAR_RECEIPT_SEEDS,
            property_system.key().as_ref(),
            candidate_pubkey.as_ref()
        ],
        bump,
        space = 8 +ArbitratorRecepit::SIZE
    )]
    pub arbitrar_receipt: Account<'info,ArbitratorRecepit>,

    #[account(
        mut,
        seeds=[
            ELECT_ARBITRAR,
            property_system.key().as_ref(),
            &proposal_id.to_le_bytes()
        ],
        bump = proposal.bump,
        constraint = proposal.snapshot_submitted @ ErrorCode::SnapshotNotSubmitted,
        constraint = proposal.status == ProposalStatus::Passed @ ErrorCode::ProposalNotPassed
    )]
    pub proposal : Account<'info,ElectAuthority>,

    #[account(
        seeds=[
            PROPERTY_SYSTEM_SEEDS,
            &property_system_id.to_le_bytes(),
        ],
        bump = property_system.bump
    )]
    pub property_system:Account<'info,PropertySystemAccount>,



    #[account(
        mut,
        seeds=[
            ARBITRAR_REGISTRYSEEDS,
            property_system.key().as_ref()
        ],
        bump = arbitrar_registry.bump 
    )]
    pub arbitrar_registry: Account<'info,ArbitratorRegistry>,


    #[account(
        seeds=[
            AUTHORITY_CANDIDATE,
            property_system.key().as_ref(), 
            proposal.key().as_ref(),
            candidate_pubkey.as_ref(),
        ],
        bump = authority_candidate.bump,
        constraint = authority_candidate.selected @ ErrorCode::AuthorityNotSelected,
        constraint = !authority_candidate.is_finalized @ ErrorCode::AlreadyFinalized
    )]
    pub authority_candidate: Account<'info,AuthorityCandidate>,

    #[account(
        mut,
        seeds=[
                CANDIDATE_PROFILE,
                candidate_pubkey.as_ref()
        ],
        bump = candidate_profile.bump,
        // constraint = candidate_profile.is_verified  @ ErrorCode::NotVerfied,
        constraint = !candidate_profile.is_blacklisted @ ErrorCode::Blacklisted 
    )]
    pub candidate_profile : Account<'info,CandidateProfile>,

    pub system_program : Program<'info,System>,

}


pub fn finalize_arbitrar_authority_candiate(
    ctx:Context<FinalizeArbitrarAuthorityCandiate>,
     _candidate_pubkey:Pubkey,_proposal_id:u64,_property_system_id:u64
)->Result<()> {

    finalize_candidate(
        &mut *ctx.accounts.proposal,
        &mut *ctx.accounts.arbitrar_registry,
        &mut ctx.accounts.authority_candidate,
        &mut ctx.accounts.candidate_profile,
    )?;
    

    Ok(())
}