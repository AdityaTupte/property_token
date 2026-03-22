use anchor_lang::prelude::*;

use crate::{common::{AUTHORITY_CANDIDATE, ELECT_TRUSTEE, PROPERTY_SYSTEM_SEEDS, ProposalStatus, TRUSTEEREGISTRYSEEDS}, errors::ErrorCode, functions::add_new_authority, state::{AuthorityCandidate, ElectAuthority, PropertySystemAccount, TrusteeRegistry}};


#[derive(Accounts)]

pub struct AddNewTrustee<'info>{

    #[account(
        constraint = trustee_registry.trustees.contains(&trustee.key()) @ ErrorCode::NotAuthorized
    )]
    pub trustee: Signer<'info>,

    #[account(
        mut,
        seeds=[
            ELECT_TRUSTEE,
            &proposal.proposal_id.to_le_bytes(),
            property_system.key().as_ref(),
        ],
        bump = proposal.bump,
        constraint = proposal.snapshot_submitted @ ErrorCode::SnapshotNotSubmitted,
        constraint = proposal.status == ProposalStatus::Passed @ ErrorCode::ProposalNotPassed
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
            TRUSTEEREGISTRYSEEDS,
            property_system.key().as_ref()
        ],
        bump = trustee_registry.bump
    )]
    pub trustee_registry : Account<'info,TrusteeRegistry>,

    #[account(
        seeds=[
            AUTHORITY_CANDIDATE,
            candidate.candidate.as_ref(),
            proposal.key().as_ref(),
            property_system.key().as_ref()
        ],
        bump = candidate.bump
    )]
    pub candidate: Account<'info,AuthorityCandidate>,

} 


pub fn add_new_trustee(
    ctx:Context<AddNewTrustee>
)->Result<()>{


    add_new_authority(
        &mut *ctx.accounts.proposal,
        &ctx.accounts.candidate.candidate,
    )?;



    

    Ok(())
}