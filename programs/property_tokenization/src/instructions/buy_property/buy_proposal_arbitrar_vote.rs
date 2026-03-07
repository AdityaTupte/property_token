use anchor_lang::prelude::*;

use crate::{common::{ARBITRAR_REGISTRYSEEDS, BUYPROPERTY, PROPERTY_SYSTEM_SEEDS, ProposalStatus}, errors::ErrorCode, functions::arbitrar_approval, state::{ArbitratorRegistry, PropertyBuyProposal, PropertySystemAccount}};


#[derive(Accounts)]

pub struct ArbitrarVote<'info>{

    #[account(
        constraint = arbitrar_registry.arbitrator.contains(&arbitrar.key()) @ ErrorCode::NotAuthorized
    )]
    pub arbitrar : Signer<'info>,


    #[account(
        mut,
        seeds=[
            BUYPROPERTY,
            proposal.buyer.key().as_ref(),
            &proposal.proposal_id.to_le_bytes()
        ],
        bump = proposal.bump,
        constraint = !proposal.is_arbitrar_approved @ ErrorCode::AlreadyApproved, 
        constraint = proposal.status == ProposalStatus::Draft @ ErrorCode::NotInDraft
    )]
    pub proposal : Account<'info,PropertyBuyProposal>,
    

    #[account(
        seeds=[
            PROPERTY_SYSTEM_SEEDS,
            &buyer.property_system_id.to_le_bytes()
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
    pub arbitrar_registry: Account<'info,ArbitratorRegistry>
}

pub fn buy_proposal_arbitrar_vote(ctx:Context<ArbitrarVote>)->Result<()>{

    let proposal_key = ctx.accounts.proposal.key();
    let proposal = &mut  *ctx.accounts.proposal;

    let signer =  ctx.accounts.arbitrar.key();

    let property_system = & ctx.accounts.buyer;

    arbitrar_approval(proposal,signer,proposal_key,property_system.governance_mint)?;

Ok(())

}