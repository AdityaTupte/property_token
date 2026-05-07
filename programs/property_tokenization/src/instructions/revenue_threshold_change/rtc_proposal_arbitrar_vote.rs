use anchor_lang::prelude::*;

use crate::{common::{ARBITRAR_RECEIPT_SEEDS, ARBITRAR_REGISTRYSEEDS, ARBITRAR_THRESOLD_CHANGE_PROPOSAL_VOTE_RECEIPT_SEEDS, PROPERTY_SYSTEM_SEEDS, ProposalStatus, RT_CHG_PROPOSAL_SEEDS}, errors::ErrorCode, functions::arbitrar_approval, state::{ArbitratorRecepit, ArbitratorRegistry, ArbitratorVoteReceipts, PropertySystemAccount, RTChgProposal}};

#[derive(Accounts)]
#[instruction(proposal_id : u64,property_system_id:u64)]
pub struct RtcArbitrarVote<'info>{

    #[account(
        mut,
    )]
    pub arbitrar : Signer<'info>,

    #[account(
        seeds = [
            ARBITRAR_RECEIPT_SEEDS,
            property_system.key().as_ref(),
            arbitrar.key().as_ref()
        ],
        bump = arbitrar_receipt.bump,
    )]
    pub arbitrar_receipt: Account<'info,ArbitratorRecepit>,


     #[account(
        seeds=[
            PROPERTY_SYSTEM_SEEDS,
            &property_system_id.to_le_bytes(),
        ],
        bump = property_system.bump,
    )]
    pub property_system : Account<'info,PropertySystemAccount>,


    #[account(
        seeds=[
            RT_CHG_PROPOSAL_SEEDS,
            property_system.key().as_ref(),
            &proposal_id.to_le_bytes(),
            
        ],
        bump= proposal.bump,
        constraint = !proposal.is_arbitrar_approved @ ErrorCode::AlreadyApproved, 
        constraint = proposal.status == ProposalStatus::Draft @ ErrorCode::NotInDraft
    )]
    pub proposal : Account<'info,RTChgProposal>,

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
        init,
        payer = arbitrar,
        seeds=[
            ARBITRAR_THRESOLD_CHANGE_PROPOSAL_VOTE_RECEIPT_SEEDS,
            property_system.key().as_ref(), 
            arbitrar.key().as_ref(), 
            proposal.key().as_ref()
        ],
        bump,
        space = 8 +ArbitratorVoteReceipts::SIZE
    )]
    
    pub arbitrar_voter: Account<'info,ArbitratorVoteReceipts>,

    
    pub system_program: Program<'info,System>

}

pub fn rtc_proposal_arbitrar_vote(
    ctx:Context<RtcArbitrarVote>,
    _proposal_id : u64,_property_system_id:u64
)->Result<()>{

    let proposal_key = ctx.accounts.proposal.key();

     let proposal = &mut  *ctx.accounts.proposal;

    let signer =  ctx.accounts.arbitrar.key();

    let property_system = & ctx.accounts.property_system;
 
    arbitrar_approval(proposal, proposal_key, &mut  ctx.accounts.arbitrar_registry, &mut ctx.accounts.arbitrar_voter, signer, property_system.governance_mint, property_system.key())?;

    Ok(())
}