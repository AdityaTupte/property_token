use anchor_lang::prelude::*;

use crate::{common::{ ARBITRAR_LEASE_PROPOSAL_VOTE_RECEIPT_SEEDS, ARBITRAR_RECEIPT_SEEDS, ARBITRAR_REGISTRYSEEDS, LEASE_PROPERTY_PROPOSAL, PROPERTY_SYSTEM_SEEDS, ProposalStatus}, errors::ErrorCode, state::{ArbitratorRecepit, ArbitratorRegistry, ArbitratorVoteReceipts, LeaseProposal, PropertySystemAccount}};


#[derive(Accounts)]
#[instruction(lease_id:u64,property:Pubkey,property_system_id:u64)]
pub struct ArbitrarApprovalForLease<'info>{


   #[account(
        mut,
        // constraint = arbitrar_registry.arbitrator.contains(&signer.key()) @ ErrorCode::NotAuthorized
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
        bump = property_system.bump
    )]
    pub property_system : Account<'info,PropertySystemAccount>,

    #[account(
        seeds=[
            ARBITRAR_REGISTRYSEEDS,
            property_system.key().as_ref()
        ],
        bump = arbitrar_registry.bump
    )]
    pub arbitrar_registry : Account<'info,ArbitratorRegistry>,

     #[account(
        init,
        payer = arbitrar,
        seeds=[
            ARBITRAR_LEASE_PROPOSAL_VOTE_RECEIPT_SEEDS,
            property_system.key().as_ref(), 
            arbitrar.key().as_ref(), 
            lease_proposal.key().as_ref()
        ],
        bump,
        space = 8 +ArbitratorVoteReceipts::SIZE
    )]
    
    pub arbitrar_voter: Account<'info,ArbitratorVoteReceipts>,



    #[account(
        mut,
        seeds=[
            LEASE_PROPERTY_PROPOSAL,
            property_system.key().as_ref(),
            property.key().as_ref(),
            &lease_id.to_le_bytes(),
            
        ],
        bump = lease_proposal.bump,
        constraint = lease_proposal.property_system == property_system.key() @ ErrorCode::PropertySystemInvalid,
        constraint = lease_proposal.status == ProposalStatus::Draft @ ErrorCode::NotInDraft
    )]
    pub lease_proposal : Account<'info,LeaseProposal>,

    pub system_program:Program<'info,System>,

}


pub fn arbitrar_approval_for_lease(
    ctx:Context<ArbitrarApprovalForLease>,
    _lease_id:u64,_property:Pubkey,_property_system_id:u64
)-> Result<()>{

    let proposal = &mut ctx.accounts.lease_proposal;

    let deadline_to_approve = proposal.initailized_at.checked_add(3*24*60*60).ok_or(ErrorCode::MathOverflow)?;

    let now: i64 = Clock::get()?.unix_timestamp ;

    require!(now <= deadline_to_approve,ErrorCode::DeadlineReached );

    proposal.arbitrar_approval_count += 1 ; 
    

    if proposal.arbitrar_approval_count >= ctx.accounts.arbitrar_registry.vote_threshold {

        proposal.status = ProposalStatus::Active;

        proposal.is_arbitrar_approved = true;

        proposal.lessee_acceptance_deadline = deadline_to_approve
                                                    .checked_add(3*24*60*60)
                                                    .ok_or(ErrorCode::MathOverflow)?;
    }

    Ok(())

}