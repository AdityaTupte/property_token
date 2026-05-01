use anchor_lang::prelude::*;

use crate::{common::{ARBITRAR_ELECTION_APPROVE_RECEIPT, ELECT_ARBITRAR, PROPERTY_SYSTEM_SEEDS, TRUSTEE_RECEIPT_SEEDS, TRUSTEEREGISTRYSEEDS}, errors::ErrorCode, functions::arbitrar_approval_for_authority, state::{ ElectAuthority, PropertySystemAccount, TrusteeRecepit, TrusteeRegistry, VoteReceiptForAuthorityElection}};

#[derive(Accounts)]
#[instruction(proposal_id:u64,property_system_id:u64)]
pub struct TrusteeApproveArbitrarElection<'info>{

    #[account(
        mut,
        // constraint = trustee_registry.trustees.contains(&signer.key()) @ ErrorCode::NotAuthorized
    )]
    pub signer : Signer<'info>,

    #[account(
        seeds = [
            TRUSTEE_RECEIPT_SEEDS,
            property_system.key().as_ref(),
            signer.key().as_ref()
        ],
        bump = trustee_receipt.bump,
    )]
    pub trustee_receipt: Account<'info,TrusteeRecepit>,

    #[account(
        seeds =[
                PROPERTY_SYSTEM_SEEDS,
                &property_system_id.to_le_bytes()
        ],
        bump = property_system.bump,
        // constraint = property_system.trustee_registry == trustee_registry.key() @ ErrorCode::PropertySystemInvalidForRegistry
    )]
    pub property_system:Account<'info,PropertySystemAccount>,

     #[account(
        seeds=[
            TRUSTEEREGISTRYSEEDS,
            property_system.key().as_ref()
        ],
        bump=trustee_registry.bump,
        constraint = trustee_registry.property_system_account == property_system.key() @ ErrorCode::ARBITRARREGISTRYINVALID
    )]
    pub trustee_registry: Account<'info,TrusteeRegistry>,


     #[account(
        mut,
        seeds=[
            ELECT_ARBITRAR,
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
            ARBITRAR_ELECTION_APPROVE_RECEIPT,
            proposal.key().as_ref(),
            signer.key().as_ref(),
        ],
        bump,
        space = 8 + VoteReceiptForAuthorityElection::SIZE ,
    )]
    pub arbitrar_voter_receipt: Account<'info,VoteReceiptForAuthorityElection>,

    pub system_program: Program<'info,System>,

} 

pub fn trustee_approve_arbitrar_election(
    ctx:Context<TrusteeApproveArbitrarElection>,
    _proposal_id:u64,
    _property_system_id:u64
)->Result<()>{

    let proposal_key = ctx.accounts.proposal.key();
    
    let proposal = &mut  *ctx.accounts.proposal;

     let voter_receipt = &mut ctx.accounts.arbitrar_voter_receipt;

    let signer =  ctx.accounts.signer.key();

    let property_system = & ctx.accounts.property_system;


    //here trustee acts as arbitrar
    arbitrar_approval_for_authority(proposal, ctx.accounts.trustee_registry.vote_threshold, voter_receipt, signer, proposal_key, ctx.accounts.property_system.governance_mint, property_system.key())?;

    Ok(())
}