use anchor_lang::prelude::*;

use crate::{constant::*, errors::ErrorCode, state::{PropertySystemAccount, TransferLandDetail, TrusteeRegistry}};


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
            TRANSFERPROPOSAL,
            property_system.key().as_ref(),
            &proposal.proposal_id.to_le_bytes(),
        ],
        bump = proposal.bump,
        constraint = proposal.source_property_system  == property_system.key(),
        close = trustee
    )]
    pub proposal : Account<'info,TransferLandDetail>,

}

pub fn delete_fail_proposal(ctx:Context<DeleteFailProposal>)->Result<()>{

    let current_time = Clock::get()?.unix_timestamp;

    let proposal = &mut ctx.accounts.proposal;

    require!(proposal.start_time  >  current_time || (proposal.end_time < current_time  && proposal.proposal_status == ProposalStatus::Failed as u8), ErrorCode::DeletingProposalInvalid);


    Ok(())

}


