use anchor_lang::prelude::*;

use crate::{common::{ARBITRAR_REGISTRYSEEDS, ELECT_TRUSTEE, PROPERTY_SYSTEM_SEEDS}, errors::ErrorCode, functions::{ arbitrar_approval_for_authority}, state::{ArbitratorRegistry, ElectAuthority, PropertySystemAccount}};

#[derive(Accounts)]
pub struct ArbitrarApproveTrusteeElection<'info>{

    #[account(
        constraint = arbitrar_registry.arbitrator.contains(&signer.key()) @ ErrorCode::NotAuthorized
    )]
    pub signer : Signer<'info>,

    #[account(
        seeds =[
                PROPERTY_SYSTEM_SEEDS,
                &property_system.property_system_id.to_le_bytes()
        ],
        bump = property_system.bump,
        constraint = property_system.arbitrator_registry == arbitrar_registry.key() @ ErrorCode::PropertySystemInvalidForRegistry
    )]
    pub property_system:Account<'info,PropertySystemAccount>,

     #[account(
        seeds=[
            ARBITRAR_REGISTRYSEEDS,
            property_system.key().as_ref()
        ],
        bump=arbitrar_registry.bump,
        constraint = arbitrar_registry.property_system_account == property_system.key() @ ErrorCode::ARBITRARREGISTRYINVALID
    )]
    pub arbitrar_registry: Account<'info,ArbitratorRegistry>,


     #[account(
        seeds=[
            ELECT_TRUSTEE,
            &proposal.proposal_id.to_le_bytes(),
            property_system.key().as_ref()
        ],
        bump = proposal.bump ,
    )]
    pub proposal : Account<'info,ElectAuthority>,

} 

pub fn arbitrar_approve_trustee_election(
    ctx:Context<ArbitrarApproveTrusteeElection>
)->Result<()>{

    let proposal_key = ctx.accounts.proposal.key();
    
    let proposal = &mut  *ctx.accounts.proposal;

    let signer =  ctx.accounts.signer.key();

    let property_system = & ctx.accounts.property_system;

    arbitrar_approval_for_authority(proposal,signer,proposal_key,property_system.governance_mint)?;

    Ok(())
}