use anchor_lang::prelude::*;

use crate::{common::{ARBITRAR_RECEIPT_SEEDS, ARBITRAR_REGISTRYSEEDS, ARBITRAR_RESIGNATION, AuthorityType, ELECT_ARBITRAR, PROPERTY_SYSTEM_SEEDS, ProposalStatus }, errors::ErrorCode, functions::finalize_authority, state::{ArbitratorRecepit, ArbitratorRegistry, ElectAuthority, PropertySystemAccount, Resignation}};

#[derive(Accounts)]
#[instruction(proposal_id:u64,property_system_id:u64,arbitrar:Pubkey)]
pub struct FinalizeOldArbitrar<'info>{

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
            ELECT_ARBITRAR,
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
            ARBITRAR_REGISTRYSEEDS,
            property_system.key().as_ref()
        ],
        bump = arbitrar_registry.bump 
    )]
    pub arbitrar_registry: Account<'info,ArbitratorRegistry>,

    #[account(
        mut,
        seeds=[
            ARBITRAR_RESIGNATION,
            property_system.key().as_ref(),
            arbitrar.as_ref(),
        ],
        bump = resignation.bump,
        constraint = resignation.status ==  ProposalStatus::Pending @ ErrorCode::AlreadyExecuted,
        constraint = resignation.authority_type == AuthorityType::ARBITRATOR @  ErrorCode::InvalidAuthorityType
    )]
    pub resignation: Account<'info,Resignation>,

}

pub fn finalize_old_arbitrar(
    ctx:Context<FinalizeOldArbitrar>,
    _proposal_id:u64,
    _property_system_id:u64,
    _arbitrar:Pubkey
)->Result<()>{

    finalize_authority(
        &mut *ctx.accounts.proposal, 
        &mut ctx.accounts.resignation, 
    )?;
    

let registry = &mut ctx.accounts.arbitrar_registry;

    registry.current_number_of_arbitrators -= 1;

    Ok(())

}