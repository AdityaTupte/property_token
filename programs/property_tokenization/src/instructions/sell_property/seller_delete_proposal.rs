use anchor_lang::prelude::*;

use crate::{constant::*, errors::ErrorCode, state::{PropertySellProposal, PropertySystemAccount, TrusteeRegistry}};


#[derive(Accounts)]
pub struct DeleteFailProposal<'info>{

    #[account(
        constraint = property_system.trustee_registry == trustee_registry.key() 
    )]
    pub trustee_registry : Account<'info,TrusteeRegistry>,

    #[account(
        mut,
        constraint = trustee_registry.trustees.contains(&trustee.key())
    )]
    pub trustee : Signer<'info>,

    #[account()]
    pub property_system : Account<'info,PropertySystemAccount>,

    #[account(
        mut,
        seeds=[
            SELLPROPERTY,
            property_system.key().as_ref(),
            &proposal.proposal_id.to_le_bytes(),
        ],
        bump = proposal.bump,
        constraint = proposal.property_system_account  == property_system.key(),
        close = trustee
    )]
    pub proposal : Account<'info,PropertySellProposal>,

}

pub fn delete_fail_proposal(ctx:Context<DeleteFailProposal>)->Result<()>{

    let current_time = Clock::get()?.unix_timestamp;

    let proposal = &mut ctx.accounts.proposal;

    require!( !proposal.snapshot_submitted, ErrorCode::AlreadyActivated);

    require!(proposal.start_time  >  current_time || (proposal.end_time < current_time  && proposal.status == ProposalStatus::Failed), ErrorCode::DeletingProposalInvalid);
   
    Ok(())

}


