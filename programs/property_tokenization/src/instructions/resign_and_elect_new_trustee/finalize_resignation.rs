use anchor_lang::prelude::*;

use crate::{common::{ELECT_TRUSTEE, PROPERTY_SYSTEM_SEEDS, ProposalStatus, TRUSTEE_RESIGNATION}, errors::ErrorCode, state::{ElectAuthority, PropertySystemAccount, Resignation}};


#[derive(Accounts)]

pub struct FinalizeTrusteeResignation<'info>{

    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds=[
            ELECT_TRUSTEE,
            &proposal.proposal_id.to_le_bytes(),
            property_system.key().as_ref(),
        ],
        bump = proposal.bump,
        constraint = proposal.status == ProposalStatus::Executed @ ErrorCode::ProposalNotExecuted
    )]
    pub proposal : Account<'info,ElectAuthority>,


    #[account(
        seeds=[
            PROPERTY_SYSTEM_SEEDS,
            &property_system.property_system_id.to_le_bytes(),
        ],
        bump = property_system.bump
    )]
    pub property_system:Account<'info,PropertySystemAccount>,

    #[account(
        seeds=[
            TRUSTEE_RESIGNATION,
            resignation.authority.as_ref(),
            property_system.key().as_ref(),
        ],
        bump = resignation.bump,
        constraint = resignation.status ==  ProposalStatus::Pending @ ErrorCode::AlreadyExecuted
    )]
    pub resignation: Account<'info,Resignation>,


}


pub fn finalize_trustee_resignation(
    ctx:Context<FinalizeTrusteeResignation>,
)->Result<()>{

    let resignation =&mut  ctx.accounts.resignation;

    require!(resignation.proposal == ctx.accounts.proposal.key(),ErrorCode::InvalidProposal);

    resignation.time = Clock::get()?.unix_timestamp;

    resignation.status = ProposalStatus::Approved;

    
    Ok(())

}