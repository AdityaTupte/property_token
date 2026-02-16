use anchor_lang::prelude::*;

use crate::state::{ArbitratorRegistry, PropertySystemAccount};
use crate::{state::TransferLandDetail};
use crate::constant::*;
use crate::errors::ErrorCode::{self};

#[derive(Accounts)]
pub struct ArbitrarApproval<'info>{

    #[account(
        constraint = arbitrar_registry.arbitrator.contains(&signer.key()) @ ErrorCode::NotAuthorized
    )]
    pub signer : Signer<'info>,

    #[account(
        mut,
        seeds=[
            TRANSFERPROPOSAL,
            &proposal.proposal_id.to_le_bytes(),
            source_property_system.key().as_ref()
        ],
        bump = proposal.bump,
        constraint = !proposal.arbitrar_approved @ ErrorCode::AlreadyApproved 
    )]
    pub proposal: Account<'info,TransferLandDetail>,


    #[account(
        seeds =[
                PROPERTY_SYSTEM_SEEDS,
                &source_property_system.property_system_id.to_le_bytes()
        ],
        bump = source_property_system.bump,
        constraint = source_property_system.arbitrator_registry == arbitrar_registry.key() @ ErrorCode::PropertySystemInvalidForRegistry
    )]
    pub source_property_system:Account<'info,PropertySystemAccount>,

    #[account(
        seeds=[
            b"arbitrator_registry",
            source_property_system.key().as_ref()
        ],
        bump=arbitrar_registry.bump,
        constraint = arbitrar_registry.property_system_account == source_property_system.key() @ ErrorCode::ARBITRARREGISTRYINVALID
    )]

    pub arbitrar_registry: Account<'info,ArbitratorRegistry>,

}

pub fn transfer_arbitrar_vote(ctx:Context<ArbitrarApproval>)->Result<()>{
    

    
    
    Ok(())
}