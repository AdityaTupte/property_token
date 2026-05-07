use anchor_lang::prelude::{ *};

use crate::{common::{ ARBITRAR_RECEIPT_SEEDS, ARBITRAR_REGISTRYSEEDS, ARBITRAR_SAFETY_PROPOSAL_VOTE_RECEIPT_SEEDS, PROPERTY_SYSTEM_SEEDS, ProposalStatus, ProposalType, SAFETYPROPOSAL, VOTERRECIEPT}, errors::ErrorCode, functions::arbitrar_approval, state::{ArbitratorRecepit, ArbitratorRegistry, ArbitratorVoteReceipts, PropertySystemAccount, TokenTransferProposal, }};

#[derive(Accounts)]
#[instruction(proposal_id : u64,property_system_id:u64)]
pub struct SafetyArbitrarVote<'info>{

    #[account(
        mut,
      //  constraint = arbitrar_registry.arbitrator.contains(&signer.key()) @ ErrorCode::NotAuthorized
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
        mut,
        seeds=[
            SAFETYPROPOSAL,
            property_system.key().as_ref(),
            &proposal_id.to_le_bytes(),
        ],
        bump = proposal.bump,
      constraint = proposal.status == ProposalStatus::Draft @ ErrorCode::NotInDraft
    )]
    pub proposal: Account<'info,TokenTransferProposal>,

    #[account(
        seeds=[
            PROPERTY_SYSTEM_SEEDS,
            &property_system_id.to_le_bytes(),
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
        bump = arbitrar_registry.bump,
        //constraint = arbitrar_registry.property_system_account == property_system.key() @ ErrorCode::ARBITRARREGISTRYINVALID
    )]
    pub arbitrar_registry: Account<'info,ArbitratorRegistry>,


    #[account(
        init,
        payer = signer,
        seeds=[
            ARBITRAR_SAFETY_PROPOSAL_VOTE_RECEIPT_SEEDS,
            property_system.key().as_ref(), 
            signer.key().as_ref(), 
            proposal.key().as_ref()
        ],
        bump,
        space = 8 +ArbitratorVoteReceipts::SIZE
    )]
    
    pub arbitrar_voter: Account<'info,ArbitratorVoteReceipts>,
    
    pub system_program:Program<'info,System>,


}

pub fn arbitrar_vote(
    ctx:Context<SafetyArbitrarVote>,
    _proposal_id : u64,_property_system_id:u64
)->Result<()>{

    let proposal_key =  ctx.accounts.proposal.key();

    let proposal = &mut *ctx.accounts.proposal;

    let signer =  ctx.accounts.signer.key();

    let property_system = &mut ctx.accounts.property_system;
    


     arbitrar_approval(
        proposal,
        proposal_key,
         &mut  ctx.accounts.arbitrar_registry,
          &mut ctx.accounts.arbitrar_voter, 
          signer,
         property_system.governance_mint, property_system.key())?;   

    Ok(())


}