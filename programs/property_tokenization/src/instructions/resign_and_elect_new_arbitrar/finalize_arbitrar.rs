use anchor_lang::prelude::*;

use crate::{common::{ARBITRAR_REGISTRYSEEDS, ARBITRAR_RESIGNATION, AuthorityType, ELECT_ARBITRAR, PROPERTY_SYSTEM_SEEDS, ProposalStatus, }, errors::ErrorCode, functions::finalize_authority, state::{ElectAuthority, PropertySystemAccount, Resignation, TrusteeRegistry}};

#[derive(Accounts)]
pub struct FinalizeArbitrar<'info>{

    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds=[
            ELECT_ARBITRAR,
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
            ARBITRAR_REGISTRYSEEDS,
            property_system.key().as_ref()
        ],
        bump = arbitrar_registry.bump 
    )]
    pub arbitrar_registry: Account<'info,TrusteeRegistry>,

    #[account(
        seeds=[
            ARBITRAR_RESIGNATION,
            resignation.authority.as_ref(),
            property_system.key().as_ref(),
        ],
        bump = resignation.bump,
        constraint = resignation.status ==  ProposalStatus::Pending @ ErrorCode::AlreadyExecuted,
        constraint = resignation.authority_type == AuthorityType::ARBITRATOR @  ErrorCode::InvalidAuthorityType
    )]
    pub resignation: Account<'info,Resignation>,

}

pub fn finalize_trustee(
    ctx:Context<FinalizeArbitrar>
)->Result<()>{

    finalize_authority(
        &mut *ctx.accounts.arbitrar_registry,
        &mut *ctx.accounts.proposal, 
        &mut ctx.accounts.resignation,
    )?;
    



    Ok(())
}