use anchor_lang::prelude::*;

use crate::{common::{AuthorityType, CHALLENGEAUTHORITY, PROPERTY_SYSTEM_SEEDS, ProposalStatus, REMOVEAUTHORITY, TRUSTEEREGISTRYSEEDS}, errors::ErrorCode, functions::finalize_authority, state::{ChallengeProposal, ElectAuthority, PropertySystemAccount, TrusteeRegistry}};

#[derive(Accounts)]
pub struct RmFinalizeTrustee<'info>{

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
        constraint = removal_proposal.status == ProposalStatus::Passed @ ErrorCode::ProposalNotPassed,
        constraint = removal_proposal.authority_type == AuthorityType::TRUSTEE,
    )]
    pub removal_proposal : Account<'info,ElectAuthority>,

    #[account(
        mut,
        seeds=[
            PROPERTY_SYSTEM_SEEDS,
            &property_system.property_system_id.to_le_bytes(),
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


}

pub fn finalize_trustee(
    ctx:Context<RmFinalizeTrustee>
)->Result<()>{

    finalize_authority(
        &mut *ctx.accounts.trustee_registry,
        &mut *ctx.accounts.removal_proposal, 
    )?;
    
     


    Ok(())
}