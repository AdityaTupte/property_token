use anchor_lang::prelude::*;

use crate::{constant::{BUYPROPOSAL, PROPERTY_SYSTEM_SEEDS}, errors::ErrorCode, events::SnapshotRequested, state::{ArbitratorRegistry, BuyLandProposalDetail, PropertySystemAccount}};


#[derive(Accounts)]

pub struct ArbitrarApproval<'info>{

    #[account(
        constraint = arbitrar_registry.arbitrator.contains(&arbitrar.key())
    )]
    pub arbitrar : Signer<'info>,

    #[account(
        mut,
        seeds=[
            BUYPROPOSAL,
            buyer.key().as_ref(),
            &proposal.proposal_id.to_le_bytes(),
        ],
        bump = proposal.bump,
        constraint = !proposal.arbitrar_approved @ ErrorCode::AlreadyApproved 
    )]
    pub proposal : Account<'info,BuyLandProposalDetail>,

    #[account(
        seeds=[
            PROPERTY_SYSTEM_SEEDS,
            &buyer.property_system_id.to_le_bytes()
        ],
        bump = buyer.bump,
        constraint = buyer.arbitrator_registry == arbitrar_registry.key() @ ErrorCode::PropertySystemInvalidForRegistry
    )]
    pub buyer: Account<'info,PropertySystemAccount>,

    #[account(
        seeds=[
            b"arbitrator_registry",
            buyer.key().as_ref()
        ],
        bump = arbitrar_registry.bump,
        constraint = arbitrar_registry.property_system_account == buyer.key() @ ErrorCode::ARBITRARREGISTRYINVALID
    )]
    pub arbitrar_registry : Account<'info,ArbitratorRegistry>,

}


 pub fn buy_land_arbitrar_vote(ctx:Context<ArbitrarApproval>)->Result<()>{

    let proposal = &mut ctx.accounts.proposal ;

    let arbitrar = & ctx.accounts.arbitrar ;

    let buyer = & ctx.accounts.buyer;

    require!(!proposal.arbitrar_approval.contains(&arbitrar.key()), ErrorCode::AuthorityApproved);

    proposal.arbitrar_approval.push(arbitrar.key());
    
    if proposal.arbitrar_approval.len() >= 3 {

        proposal.arbitrar_approved = true;

        let slot = Clock::get()?.slot;

        emit!(SnapshotRequested{
            proposal_id:proposal.proposal_id,
            mint : buyer.governance_mint,
            slot : slot,
        })
    }

    Ok(())
 }