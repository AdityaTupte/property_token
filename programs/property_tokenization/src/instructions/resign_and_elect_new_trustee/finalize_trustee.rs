use anchor_lang::prelude::*;

use crate::{common::{AuthorityType, ELECT_TRUSTEE, PROPERTY_SYSTEM_SEEDS, ProposalStatus, TRUSTEE_RECEIPT_SEEDS, TRUSTEE_RESIGNATION, TRUSTEEREGISTRYSEEDS}, errors::ErrorCode, functions::finalize_authority, state::{ElectAuthority, PropertySystemAccount, Resignation, TrusteeRecepit, TrusteeRegistry}};

#[derive(Accounts)]
#[instruction(proposal_id:u64,property_system_id:u64,trustee:Pubkey)]
pub struct FinalizeTrustee<'info>{

    #[account(mut)]
    pub signer: Signer<'info>,


    #[account(
        mut,
        seeds = [
            TRUSTEE_RECEIPT_SEEDS,
            property_system.key().as_ref(),
            trustee.key().as_ref()
        ],
        bump = trustee_receipt.bump,
        constraint = property_system.key() == trustee_receipt.property_system_account @ ErrorCode::PropertySystemInvalidForRegistry,
        close = signer,
    )]
    pub trustee_receipt: Account<'info,TrusteeRecepit>,

    #[account(
        mut,
        seeds=[
             ELECT_TRUSTEE,
            property_system.key().as_ref(),
            &proposal_id.to_le_bytes(),
        ],
        bump = proposal.bump,
        constraint = proposal.snapshot_submitted @ ErrorCode::SnapshotNotSubmitted,
        constraint = proposal.status == ProposalStatus::Active @ ErrorCode::ProposalNotActive
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
            TRUSTEE_RESIGNATION,
            property_system.key().as_ref(),
            trustee.as_ref(), 
        ],
        bump = resignation.bump,
        constraint = resignation.status ==  ProposalStatus::Pending @ ErrorCode::AlreadyExecuted,
        constraint = resignation.authority_type == AuthorityType::TRUSTEE @ ErrorCode::InvalidAuthorityType
    )]
    pub resignation: Account<'info,Resignation>,

}

pub fn finalize_trustee(
    ctx:Context<FinalizeTrustee>,
    _proposal_id:u64,
    _property_system_id:u64,
    _trustee:Pubkey
)->Result<()>{

    finalize_authority(
        &mut *ctx.accounts.proposal, 
        &mut ctx.accounts.resignation,
    )?;
    
    let registry = &mut ctx.accounts.trustee_registry;

    registry.current_number_of_trustees -= 1;

    Ok(())
}