use anchor_lang::prelude::*;

use crate::{common::{ARBITRAR_RECEIPT_SEEDS, ARBITRAR_REGISTRYSEEDS, ELECT_TRUSTEE, PROPERTY_SYSTEM_SEEDS, TRUSTEE_ELECTION_APPROVE_RECEIPT}, errors::ErrorCode, functions::arbitrar_approval_for_authority, state::{ArbitratorRecepit, ArbitratorRegistry, ElectAuthority, PropertySystemAccount, VoteReceiptForAuthorityElection}};

#[derive(Accounts)]
#[instruction(proposal_id:u64,property_system_id:u64)]
pub struct ArbitrarApproveTrusteeElection<'info>{

    #[account(
        mut,
        // constraint = arbitrar_registry.arbitrator.contains(&signer.key()) @ ErrorCode::NotAuthorized
    )]
    pub signer : Signer<'info>,

     #[account(
        seeds = [
            ARBITRAR_RECEIPT_SEEDS,
            property_system.key().as_ref(),
            signer.key().as_ref()
        ],
        bump = arbitrar_receipt.bump,
    )]
    pub arbitrar_receipt: Account<'info,ArbitratorRecepit>,

    #[account(
        seeds =[
                PROPERTY_SYSTEM_SEEDS,
                &property_system_id.to_le_bytes()
        ],
        bump = property_system.bump,
        // constraint = property_system.arbitrator_registry == arbitrar_registry.key() @ ErrorCode::PropertySystemInvalidForRegistry
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
        mut,
        seeds=[
            ELECT_TRUSTEE,
            property_system.key().as_ref(),
            &proposal_id.to_le_bytes(),
        ],
        bump = proposal.bump ,
    )]
    pub proposal : Account<'info,ElectAuthority>,


    #[account(
        init,
        payer = signer,
        seeds=[
            TRUSTEE_ELECTION_APPROVE_RECEIPT,
            proposal.key().as_ref(),
            signer.key().as_ref(),
        ],
        bump,
        space = 8 + VoteReceiptForAuthorityElection::SIZE ,
    )]
    pub arbitrar_voter_receipt: Account<'info,VoteReceiptForAuthorityElection>,

    pub system_program: Program<'info,System>,

} 

pub fn arbitrar_approve_trustee_election(
    ctx:Context<ArbitrarApproveTrusteeElection>,
    _proposal_id:u64,
    _property_system_id:u64
)->Result<()>{

    let proposal_key = ctx.accounts.proposal.key();
    
    let proposal = &mut  *ctx.accounts.proposal;

    let voter_receipt = &mut ctx.accounts.arbitrar_voter_receipt;

    let signer =  ctx.accounts.signer.key();

    let property_system = & ctx.accounts.property_system;

    arbitrar_approval_for_authority(proposal, ctx.accounts.arbitrar_registry.vote_threshold, voter_receipt, signer, proposal_key, ctx.accounts.property_system.governance_mint, property_system.key())?;

    Ok(())
}