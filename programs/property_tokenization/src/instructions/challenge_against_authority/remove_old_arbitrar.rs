use anchor_lang::prelude::*;

use crate::{common::{ARBITRAR_RECEIPT_SEEDS, ARBITRAR_REGISTRYSEEDS, AuthorityType, CANDIDATE_PROFILE,  PROPERTY_SYSTEM_SEEDS, ProposalStatus, REMOVEARBITRARAUTHORITY, REMOVEARBITRARAUTHORITYPROPOSAL,}, errors::ErrorCode, functions::finalize_authority, state::{ArbitratorRecepit, ArbitratorRegistry, CandidateProfile, ElectAuthority, PropertySystemAccount, Resignation,}};

#[derive(Accounts)]
#[instruction(proposal_id:u64,proposal_key:Pubkey,property_system_id:u64,arbitrar:Pubkey)]
pub struct RemoveOldArbitrar<'info>{

    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [
            ARBITRAR_RECEIPT_SEEDS,
            property_system.key().as_ref(),
            arbitrar.as_ref()
        ],
        bump = arbitrar_receipt.bump,
        constraint = property_system.key() == arbitrar_receipt.property_system_account @ ErrorCode::PropertySystemInvalidForRegistry,
        close = signer,
    )]
    pub arbitrar_receipt: Account<'info,ArbitratorRecepit>,

   #[account(
        mut,
        seeds=[
                CANDIDATE_PROFILE,
                arbitrar.as_ref()
        ],
        bump = candidate_profile.bump,
        // constraint = candidate_profile.is_verified  @ ErrorCode::NotVerfied,
        // constraint = !candidate_profile.is_blacklisted @ ErrorCode::Blacklisted 
    )]
    pub candidate_profile : Account<'info,CandidateProfile>,


    #[account(
        mut,
        seeds=[
            REMOVEARBITRARAUTHORITYPROPOSAL,
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
            ARBITRAR_REGISTRYSEEDS,
            property_system.key().as_ref()
        ],
        bump = arbitrar_registry.bump 
    )]
    pub arbitrar_registry: Account<'info,ArbitratorRegistry>,

    #[account(
        mut,
        seeds=[
            REMOVEARBITRARAUTHORITY,
            property_system.key().as_ref(),
            arbitrar.as_ref(), 
        ],
        bump = resignation.bump,
        constraint = resignation.status ==  ProposalStatus::Pending @ ErrorCode::AlreadyExecuted,
        constraint = resignation.authority_type == AuthorityType::ARBITRATOR @ ErrorCode::InvalidAuthorityType
    )]
    pub resignation: Account<'info,Resignation>,

}

pub fn remove_old_arbitrar(
    ctx:Context<RemoveOldArbitrar>,
    _proposal_id:u64,
    _proposal_key:Pubkey,
    _property_system_id:u64,
    _arbitrar:Pubkey
)->Result<()>{

    finalize_authority(
        &mut *ctx.accounts.proposal, 
        &mut ctx.accounts.resignation,
    )?;
    let candidate_profile = &mut ctx.accounts.candidate_profile;

    candidate_profile.removal_count +=1;
    
    let registry = &mut ctx.accounts.arbitrar_registry;

    

    registry.current_number_of_arbitrators -= 1;

    Ok(())
}