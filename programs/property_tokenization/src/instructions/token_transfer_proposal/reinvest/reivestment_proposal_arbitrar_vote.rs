use anchor_lang::prelude::{ *};

use crate::{constant::{PROPERTY_SYSTEM_SEEDS, USEREINVESTMENTOKEN}, errors::ErrorCode, functions::arbitrar_approval, state::{ArbitratorRegistry, PropertySystemAccount, UseReinvestmentProposal}};

#[derive(Accounts)]
pub struct ArbitrarVote<'info>{

    #[account(constraint = arbitrar_registry.arbitrator.contains(&signer.key()) @ ErrorCode::NotAuthorized)]
    pub signer : Signer<'info>,

    #[account(
        seeds=[
            USEREINVESTMENTOKEN,
            property_system.key().as_ref(),
            &proposal.proposal_id.to_le_bytes(),
        ],
        bump = proposal.bump,
        constraint = proposal.property_system == property_system.key() @ ErrorCode::InvalidProposal 
    )]
    pub proposal: Account<'info,UseReinvestmentProposal>,

    #[account(
        seeds=[
            PROPERTY_SYSTEM_SEEDS,
            &property_system.property_system_id.to_le_bytes(),
        ],
        bump = property_system.bump,
        constraint = property_system.arbitrator_registry == arbitrar_registry.key() @ ErrorCode::PropertySystemInvalidForRegistry
    )]
    pub property_system:Account<'info,PropertySystemAccount>,

    #[account(
        seeds=[
            b"arbitrator_registry",
            property_system.key().as_ref()
        ],
        bump = arbitrar_registry.bump,
        constraint = arbitrar_registry.property_system_account == property_system.key() @ ErrorCode::ARBITRARREGISTRYINVALID
    )]
    pub arbitrar_registry: Account<'info,ArbitratorRegistry>,

}

pub fn arbitrar_vote(
    ctx:Context<ArbitrarVote>,
)->Result<()>{

    let proposal_key =  ctx.accounts.proposal.key();

    let proposal = &mut *ctx.accounts.proposal;

    let signer =  ctx.accounts.signer.key();

    let property_system = &mut ctx.accounts.property_system;
    

    arbitrar_approval(proposal, signer, proposal_key, property_system.governance_mint)?;

    Ok(())


}