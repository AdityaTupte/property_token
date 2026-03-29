use anchor_lang::prelude::*;

use crate::{common::{ARBITRAR_REGISTRYSEEDS, LEASE_PROPERTY_PROPOSAL, PROPERTY_SYSTEM_SEEDS, ProposalStatus}, errors::ErrorCode, state::{ArbitratorRegistry, LeaseProposal, PropertySystemAccount}};


#[derive(Accounts)]

pub struct ArbitrarApprovalForLease<'info>{


   #[account(
        mut,
        constraint = arbitrar_registry.arbitrator.contains(&signer.key()) @ ErrorCode::NotAuthorized
    )]
    pub signer : Signer<'info>,

    #[account(
        seeds=[
            PROPERTY_SYSTEM_SEEDS,
            &property_system.property_system_id.to_le_bytes(),
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
    
        seeds=[
            LEASE_PROPERTY_PROPOSAL,
            &lease_proposal.lease_id.to_le_bytes(),
            lease_proposal.property.key().as_ref(),
        ],
        bump = lease_proposal.bump,
        constraint = lease_proposal.property_system == property_system.key() @ ErrorCode::PropertySystemInvalid,
        constraint = lease_proposal.status == ProposalStatus::Draft @ ErrorCode::NotInDraft
    )]
    pub lease_proposal : Account<'info,LeaseProposal>,

}


pub fn arbitrar_approval_for_lease(
    ctx:Context<ArbitrarApprovalForLease>
)-> Result<()>{

    let proposal = &mut ctx.accounts.lease_proposal;

    let deadline_to_approve = proposal.initailized_at.checked_add(3*24*60*60).ok_or(ErrorCode::MathOverflow)?;

    let now: i64 = Clock::get()?.unix_timestamp ;

    require!(now <= deadline_to_approve,ErrorCode::DeadlineReached );

    let signer = ctx.accounts.signer.key();

    require!(
        !proposal.arbitrar_approval.contains(&signer),
        ErrorCode::AuthorityApproved
    );
    
    proposal.arbitrar_approval.push(signer);

    if proposal.arbitrar_approval.len() >= 3 {

        proposal.status = ProposalStatus::Active;

        proposal.lessee_acceptance_deadline = deadline_to_approve
                                                    .checked_add(3*24*60*60)
                                                    .ok_or(ErrorCode::MathOverflow)?;
    }

    Ok(())

}