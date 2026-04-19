use anchor_lang::prelude::*;

use crate::common::{ARBITRAR_RECEIPT_SEEDS, ARBITRAR_REGISTRYSEEDS, ARBITRAR_SELL_PROPOSAL_VOTE_RECEIPT_SEEDS, PROPERTY_SYSTEM_SEEDS, ProposalStatus, SELLPROPERTY};
use crate::functions::{arbitrar_approval};
use crate::state::{ArbitratorRecepit, ArbitratorRegistry, ArbitratorVoteReceipts, PropertySellProposal, PropertySystemAccount};

use crate::errors::ErrorCode::{self};

#[derive(Accounts)]
#[instruction(proposal_id : u64,property_system_id:u64)]
pub struct ArbitrarApproval<'info>{

    #[account(
        mut,
    )]
    pub arbitrar : Signer<'info>,

     #[account(
        seeds = [
            ARBITRAR_RECEIPT_SEEDS,
            seller.key().as_ref(),
            arbitrar.key().as_ref()
        ],
        bump = arbitrar_receipt.bump,
    )]
    pub arbitrar_receipt: Account<'info,ArbitratorRecepit>,


    #[account(
        mut,
        seeds=[
            SELLPROPERTY,
            seller.key().as_ref(),
            &proposal_id.to_le_bytes(),
    
        ],
        bump = proposal.bump,
        constraint = !proposal.is_arbitrar_approved @ ErrorCode::AlreadyApproved, 
        constraint = proposal.status == ProposalStatus::Draft @ ErrorCode::NotInDraft
        )]
    pub proposal: Account<'info,PropertySellProposal>,


    #[account(
        seeds =[
                PROPERTY_SYSTEM_SEEDS,
                &property_system_id.to_le_bytes()
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

    #[account(
        init,
        payer = arbitrar,
        seeds=[
            ARBITRAR_SELL_PROPOSAL_VOTE_RECEIPT_SEEDS,
            seller.key().as_ref(), 
            arbitrar.key().as_ref(), 
            proposal.key().as_ref()
        ],
        bump,
        space = 8 +ArbitratorVoteReceipts::SIZE
    )]
    
    pub arbitrar_voter: Account<'info,ArbitratorVoteReceipts>,
    
    pub system_program: Program<'info,System>

}

pub fn sell_proposal_arbitrar_vote(ctx:Context<ArbitrarApproval>,_proposal_id : u64,_property_system_id:u64)->Result<()>{
    
    
    
    let proposal = &mut  ctx.accounts.proposal;

    let signer =  ctx.accounts.arbitrar.key();

    let property_system = & ctx.accounts.seller;

   

     
    arbitrar_approval(proposal, &mut  ctx.accounts.arbitrar_registry, &mut ctx.accounts.arbitrar_voter, signer, property_system.governance_mint)?;

    Ok(())
}