use anchor_lang::prelude::*;

use crate::{common::{ARBITRAR_REGISTRYSEEDS, ELECT_ARBITRAR, PROPERTY_SYSTEM_SEEDS}, errors::ErrorCode, functions::arbitrar_approval_for_authority, state::{ ElectAuthority, PropertySystemAccount, TrusteeRegistry}};

#[derive(Accounts)]
pub struct TrusteeApproveArbitrarElection<'info>{

    #[account(
        constraint = trustee_registry.trustees.contains(&signer.key()) @ ErrorCode::NotAuthorized
    )]
    pub signer : Signer<'info>,

    #[account(
        seeds =[
                PROPERTY_SYSTEM_SEEDS,
                &property_system.property_system_id.to_le_bytes()
        ],
        bump = property_system.bump,
        constraint = property_system.trustee_registry == trustee_registry.key() @ ErrorCode::PropertySystemInvalidForRegistry
    )]
    pub property_system:Account<'info,PropertySystemAccount>,

     #[account(
        seeds=[
            ARBITRAR_REGISTRYSEEDS,
            property_system.key().as_ref()
        ],
        bump=trustee_registry.bump,
        constraint = trustee_registry.property_system_account == property_system.key() @ ErrorCode::ARBITRARREGISTRYINVALID
    )]
    pub trustee_registry: Account<'info,TrusteeRegistry>,


     #[account(
        seeds=[
            ELECT_ARBITRAR,
            &proposal.proposal_id.to_le_bytes(),
            property_system.key().as_ref()
        ],
        bump = proposal.bump ,
    )]
    pub proposal : Account<'info,ElectAuthority>,

} 

pub fn trustee_approve_arbitrar_election(
    ctx:Context<TrusteeApproveArbitrarElection>
)->Result<()>{

    let proposal_key = ctx.accounts.proposal.key();
    
    let proposal = &mut  *ctx.accounts.proposal;

    let signer =  ctx.accounts.signer.key();

    let property_system = & ctx.accounts.property_system;


    //here trustee acts as arbitrar
    arbitrar_approval_for_authority(proposal,signer,proposal_key,property_system.governance_mint)?;

    Ok(())
}