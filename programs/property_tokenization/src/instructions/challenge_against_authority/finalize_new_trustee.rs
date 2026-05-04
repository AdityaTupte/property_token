use anchor_lang::prelude::*;

use crate::{common::{ AUTHORITY_CANDIDATE, CANDIDATE_PROFILE,  PROPERTY_SYSTEM_SEEDS, ProposalStatus,  REMOVETRUSTEEAUTHORITYPROPOSAL, TRUSTEE_RECEIPT_SEEDS, TRUSTEEREGISTRYSEEDS}, errors::ErrorCode, functions::{ finalized_candidate_for_remove_proposal}, state::{  AuthorityCandidate, CandidateProfile, ElectAuthority, PropertySystemAccount, TrusteeRecepit, TrusteeRegistry}};



#[derive(Accounts)]
#[instruction(candidate_pubkey:Pubkey,proposal_id:u64,property_system_id:u64,proposal_key:Pubkey)]
pub struct FinalizeNewTrustee<'info>{

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
            TRUSTEE_RECEIPT_SEEDS,
            property_system.key().as_ref(),
            candidate_pubkey.as_ref()
        ],
        bump,
        space = 8 +TrusteeRecepit::SIZE
    )]
    pub trustee_receipt: Account<'info,TrusteeRecepit>,

    #[account(
        mut,
        seeds=[
            REMOVETRUSTEEAUTHORITYPROPOSAL,
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
            TRUSTEEREGISTRYSEEDS,
            property_system.key().as_ref()
        ],
        bump = trustee_registry.bump 
    )]
    pub trustee_registry: Account<'info,TrusteeRegistry>,


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
        constraint = !candidate_profile.is_blacklisted @ ErrorCode::Blacklisted 
    )]
    pub candidate_profile : Account<'info,CandidateProfile>,

    pub system_program : Program<'info,System>,

}


pub fn finalize_new_trustee(
    ctx:Context<FinalizeNewTrustee>,
     _candidate_pubkey:Pubkey,_proposal_id:u64,_property_system_id:u64,_proposal_key:Pubkey
)->Result<()> {

    finalized_candidate_for_remove_proposal(
        &mut *ctx.accounts.removal_proposal,
        &mut *ctx.accounts.trustee_registry,
        &mut ctx.accounts.authority_candidate,
        &mut ctx.accounts.candidate_profile,
    )?;
    
    let trustee_receipt = &mut ctx.accounts.trustee_receipt;

    trustee_receipt.property_system_account = ctx.accounts.property_system.key();

    trustee_receipt.trustee = ctx.accounts.candidate.key();

    trustee_receipt.bump = ctx.bumps.trustee_receipt;


    Ok(())
}