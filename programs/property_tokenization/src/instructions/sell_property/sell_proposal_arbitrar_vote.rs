use anchor_lang::prelude::*;

use crate::common::{ARBITRAR_REGISTRYSEEDS, PROPERTY_SYSTEM_SEEDS, ProposalStatus, SELLPROPERTY};
use crate::functions::{arbitrar_approval};
use crate::state::{ArbitratorRegistry, PropertySystemAccount, PropertySellProposal};

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
            SELLPROPERTY,
            seller.key().as_ref(),
            &proposal.proposal_id.to_le_bytes(),
    
        ],
        bump = proposal.bump,
        constraint = !proposal.is_arbitrar_approved @ ErrorCode::AlreadyApproved, 
        constraint = proposal.status == ProposalStatus::Draft @ ErrorCode::NotInDraft
        )]
    pub proposal: Account<'info,PropertySellProposal>,


    #[account(
        seeds =[
                PROPERTY_SYSTEM_SEEDS,
                &seller.property_system_id.to_le_bytes()
        ],
        bump = seller.bump,
        constraint = seller.arbitrator_registry == arbitrar_registry.key() @ ErrorCode::PropertySystemInvalidForRegistry
    )]
    pub seller:Account<'info,PropertySystemAccount>,

    #[account(
        seeds=[
            ARBITRAR_REGISTRYSEEDS,
            seller.key().as_ref()
        ],
        bump=arbitrar_registry.bump,
        constraint = arbitrar_registry.property_system_account == seller.key() @ ErrorCode::ARBITRARREGISTRYINVALID
    )]

    pub arbitrar_registry: Account<'info,ArbitratorRegistry>,

}

pub fn sell_proposal_arbitrar_vote(ctx:Context<ArbitrarApproval>)->Result<()>{
    
    let proposal_key = ctx.accounts.proposal.key();
    
    let proposal = &mut  *ctx.accounts.proposal;

    let signer =  ctx.accounts.signer.key();

    let property_system = & ctx.accounts.seller;

    arbitrar_approval(proposal,signer,proposal_key,property_system.governance_mint)?;

    Ok(())
}