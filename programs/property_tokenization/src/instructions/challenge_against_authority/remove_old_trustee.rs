use anchor_lang::prelude::*;

use crate::{common::{AuthorityType, CANDIDATE_PROFILE, PROPERTY_SYSTEM_SEEDS, ProposalStatus,  REMOVETRUSTEEAUTHORITY, REMOVETRUSTEEAUTHORITYPROPOSAL, TRUSTEE_RECEIPT_SEEDS,  TRUSTEEREGISTRYSEEDS}, errors::ErrorCode, functions::finalize_authority, state::{CandidateProfile, ElectAuthority, PropertySystemAccount, Resignation, TrusteeRecepit, TrusteeRegistry, }};

#[derive(Accounts)]
#[instruction(proposal_id:u64,proposal_key:Pubkey,property_system_id:u64,trustee:Pubkey)]
pub struct RemoveOldTrustee<'info>{

    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [
            TRUSTEE_RECEIPT_SEEDS,
            property_system.key().as_ref(),
            trustee.as_ref()
        ],
        bump = trustee_receipt.bump,
        constraint = property_system.key() == trustee_receipt.property_system_account @ ErrorCode::PropertySystemInvalidForRegistry,
        close = signer,
    )]
    pub trustee_receipt: Account<'info,TrusteeRecepit>,

   #[account(
        mut,
        seeds=[
                CANDIDATE_PROFILE,
                trustee.as_ref()
        ],
        bump = candidate_profile.bump,
        // constraint = candidate_profile.is_verified  @ ErrorCode::NotVerfied,
        // constraint = !candidate_profile.is_blacklisted @ ErrorCode::Blacklisted 
    )]
    pub candidate_profile : Account<'info,CandidateProfile>,


    #[account(
        mut,
        seeds=[
            REMOVETRUSTEEAUTHORITYPROPOSAL,
            property_system.key().as_ref(),
            proposal_key.as_ref(),
        ],
        bump = proposal.bump,
        constraint = proposal.snapshot_submitted @ ErrorCode::SnapshotNotSubmitted,
        constraint = proposal.status == ProposalStatus::Approved @ ErrorCode::ProposalNotApproved
    )]
    pub proposal : Account<'info,ElectAuthority>,

    #[account(
        mut,
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
            REMOVETRUSTEEAUTHORITY,
            property_system.key().as_ref(),
            trustee.key().as_ref(), 
        ],
        bump = resignation.bump,
        constraint = resignation.status ==  ProposalStatus::Pending @ ErrorCode::AlreadyExecuted,
        constraint = resignation.authority_type == AuthorityType::TRUSTEE @ ErrorCode::InvalidAuthorityType
    )]
    pub resignation: Account<'info,Resignation>,

}

pub fn remove_old_trustee(
    ctx:Context<RemoveOldTrustee>,
    _proposal_id:u64,
    _proposal_key:Pubkey,
    _property_system_id:u64,
    _trustee:Pubkey
)->Result<()>{

    finalize_authority(
        &mut *ctx.accounts.proposal, 
        &mut ctx.accounts.resignation,
    )?;
    let candidate_profile = &mut ctx.accounts.candidate_profile;

    candidate_profile.removal_count +=1;
    
    let registry = &mut ctx.accounts.trustee_registry;

    registry.current_number_of_trustees -= 1;

    Ok(())
}