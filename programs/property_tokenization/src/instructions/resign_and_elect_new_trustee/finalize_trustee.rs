use anchor_lang::prelude::*;

use crate::{common::{ELECT_TRUSTEE, PROPERTY_SYSTEM_SEEDS, ProposalStatus, TRUSTEEREGISTRYSEEDS}, errors::ErrorCode, functions::finalize_authority, state::{ElectAuthority, PropertySystemAccount, TrusteeRegistry}};

#[derive(Accounts)]
pub struct FinalizeTrustee<'info>{

    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds=[
            ELECT_TRUSTEE,
            &proposal.proposal_id.to_le_bytes(),
            property_system.key().as_ref(),
        ],
        bump = proposal.bump,
        constraint = proposal.snapshot_submitted @ ErrorCode::SnapshotNotSubmitted,
        constraint = proposal.status == ProposalStatus::Passed @ ErrorCode::ProposalNotPassed
    )]
    pub proposal : Account<'info,ElectAuthority>,

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
    ctx:Context<FinalizeTrustee>
)->Result<()>{

    finalize_authority(
        &mut *ctx.accounts.trustee_registry,
        &mut *ctx.accounts.proposal, 
    )?;
    
    Ok(())
}