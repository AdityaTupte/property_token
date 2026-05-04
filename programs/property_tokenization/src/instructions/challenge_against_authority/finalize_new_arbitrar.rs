use anchor_lang::prelude::*;

use crate::{common::{ARBITRAR_RECEIPT_SEEDS, ARBITRAR_REGISTRYSEEDS, AUTHORITY_CANDIDATE, CANDIDATE_PROFILE,  PROPERTY_SYSTEM_SEEDS, ProposalStatus, REMOVEARBITRARAUTHORITYPROPOSAL,   }, errors::ErrorCode, functions::{ finalized_candidate_for_remove_proposal}, state::{ArbitratorRecepit, ArbitratorRegistry, AuthorityCandidate, CandidateProfile, ElectAuthority, PropertySystemAccount, }};



#[derive(Accounts)]
#[instruction(candidate_pubkey:Pubkey,proposal_id:u64,property_system_id:u64,proposal_key:Pubkey)]
pub struct FinalizeNewArbitrar<'info>{

    #[account(
        mut
    )]
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
            REMOVEARBITRARAUTHORITYPROPOSAL,
            property_system.key().as_ref(),
            proposal_key.as_ref(),
        ],
        bump = removal_proposal.bump,
        constraint = removal_proposal.snapshot_submitted @ ErrorCode::SnapshotNotSubmitted,
        constraint = removal_proposal.status == ProposalStatus::Passed @ ErrorCode::ProposalNotPassed
    )]
    pub removal_proposal : Account<'info,ElectAuthority>,

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
        mut,
        seeds=[
            AUTHORITY_CANDIDATE,
            property_system.key().as_ref(), 
            removal_proposal.key().as_ref(),
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
        // constraint = !candidate_profile.is_blacklisted @ ErrorCode::Blacklisted 
    )]
    pub candidate_profile : Account<'info,CandidateProfile>,

    pub system_program : Program<'info,System>,

}


pub fn finalize_new_arbitrar(
    ctx:Context<FinalizeNewArbitrar>,
     _candidate_pubkey:Pubkey,_proposal_id:u64,_property_system_id:u64,_proposal_key:Pubkey
)->Result<()> {

    finalized_candidate_for_remove_proposal(
        &mut *ctx.accounts.removal_proposal,
        &mut *ctx.accounts.arbitrar_registry,
        &mut ctx.accounts.authority_candidate,
        &mut ctx.accounts.candidate_profile,
    )?;
    
    let arbitrar_receipt = &mut ctx.accounts.arbitrar_receipt;

    arbitrar_receipt.property_system_account = ctx.accounts.property_system.key();

    arbitrar_receipt.arbitrator = ctx.accounts.candidate.key();

    arbitrar_receipt.bump = ctx.bumps.arbitrar_receipt;


    Ok(())
}