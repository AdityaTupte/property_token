use anchor_lang::prelude::*;

use crate::events::SnapshotRequested;
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
            source_property_system.key().as_ref(),
            &proposal.proposal_id.to_le_bytes(),
    
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
    

    let proposal = &mut  ctx.accounts.proposal;

    let signer = & ctx.accounts.signer;

    let property_sysytem = & ctx.accounts.source_property_system;

    require!(!proposal.arbitrar_approval.contains(&signer.key()), ErrorCode::AuthorityApproved);

    proposal.arbitrar_approval.push(signer.key());

    if proposal.arbitrar_approval.len() >= 3 {

        proposal.arbitrar_approved = true;

        let slot = Clock::get()?.slot;
        
        emit!(SnapshotRequested{
            proposal_id : proposal.proposal_id,
            mint : property_sysytem.governance_mint,
            slot : slot,

        })
        
    }

    Ok(())
}