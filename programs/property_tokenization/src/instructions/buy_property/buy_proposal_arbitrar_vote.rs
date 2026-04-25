use anchor_lang::prelude::*;

use crate::{common::{ARBITRAR_BUY_PROPOSAL_VOTE_RECEIPT_SEEDS, ARBITRAR_RECEIPT_SEEDS, ARBITRAR_REGISTRYSEEDS, BUYPROPERTY, PROPERTY_SYSTEM_SEEDS, ProposalStatus}, errors::ErrorCode, functions::arbitrar_approval, state::{ArbitratorRecepit, ArbitratorRegistry, ArbitratorVoteReceipts, PropertyBuyProposal, PropertySystemAccount}};


#[derive(Accounts)]
#[instruction(proposal_id : u64,buyer_property_system_id:u64)]
pub struct ArbitrarVote<'info>{

    #[account(
        mut,
        //constraint = arbitrar_registry.arbitrator.contains(&arbitrar.key()) @ ErrorCode::NotAuthorized
    )]
    pub arbitrar : Signer<'info>,

     #[account(
        seeds = [
            ARBITRAR_RECEIPT_SEEDS,
            buyer.key().as_ref(),
            arbitrar.key().as_ref()
        ],
        bump = arbitrar_receipt.bump,
    )]
    pub arbitrar_receipt: Account<'info,ArbitratorRecepit>,


    #[account(
        mut,
        seeds=[
            BUYPROPERTY,
            buyer.key().as_ref(),
            &proposal_id.to_le_bytes()
        ],
        bump = proposal.bump,
        constraint = !proposal.is_arbitrar_approved @ ErrorCode::AlreadyApproved, 
        constraint = proposal.status == ProposalStatus::Draft @ ErrorCode::NotInDraft
    )]
    pub proposal : Account<'info,PropertyBuyProposal>,
    

    #[account(
        seeds=[
            PROPERTY_SYSTEM_SEEDS,
            &buyer_property_system_id.to_le_bytes()
        ],
        bump= buyer.bump,
        //constraint = buyer.arbitrator_registry == arbitrar_registry.key() @ ErrorCode::PropertySystemInvalidForRegistry
    )]
    pub buyer:Account<'info,PropertySystemAccount>,

    #[account(
        seeds=[
            ARBITRAR_REGISTRYSEEDS,
            buyer.key().as_ref()
        ],
        bump=arbitrar_registry.bump,
        //constraint = arbitrar_registry.property_system_account == buyer.key() @ ErrorCode::ARBITRARREGISTRYINVALID
    )]
    pub arbitrar_registry: Account<'info,ArbitratorRegistry>,

     #[account(
        init,
        payer = arbitrar,
        seeds=[
            ARBITRAR_BUY_PROPOSAL_VOTE_RECEIPT_SEEDS,
            buyer.key().as_ref(), 
            arbitrar.key().as_ref(), 
            proposal.key().as_ref()
        ],
        bump,
        space = 8 +ArbitratorVoteReceipts::SIZE
    )]
    
    pub arbitrar_voter: Account<'info,ArbitratorVoteReceipts>,
    
    pub system_program: Program<'info,System>


}

pub fn buy_proposal_arbitrar_vote(
    ctx:Context<ArbitrarVote>,
    _proposal_id : u64,
    _buyer_property_system_id:u64
)->Result<()>{

    let proposal_key = ctx.accounts.proposal.key();

     let proposal = &mut  *ctx.accounts.proposal;

    let signer =  ctx.accounts.arbitrar.key();

    let property_system = & ctx.accounts.buyer;

     
    arbitrar_approval(proposal, proposal_key, &mut  ctx.accounts.arbitrar_registry, &mut ctx.accounts.arbitrar_voter, signer, property_system.governance_mint, property_system.key())?;

Ok(())

}