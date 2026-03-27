use anchor_lang::prelude::*;

use crate::{common::{AUTHORITY_CANDIDATE, AuthorityType, CHALLENGEAUTHORITY, PROPERTY_SYSTEM_SEEDS, ProposalStatus, REMOVEAUTHORITY, TRUSTEEREGISTRYSEEDS}, errors::ErrorCode, functions::add_new_authority, state::{AuthorityCandidate, ChallengeProposal, ElectAuthority, PropertySystemAccount, TrusteeRegistry}};


#[derive(Accounts)]
pub struct AddNewAuthorityAsTrustee<'info>{

    pub signer: Signer<'info>,

      #[account(
        mut,
        seeds=[
            REMOVEAUTHORITY,
            proposal.key().as_ref(),
            property_system.key().as_ref(),
        ],
        bump = removal_proposal.bump,
        constraint = removal_proposal.snapshot_submitted @ ErrorCode::SnapshotNotSubmitted,
        constraint = removal_proposal.status == ProposalStatus::Passed @ ErrorCode::ProposalNotPassed,
        constraint = removal_proposal.authority_type == AuthorityType::TRUSTEE
    )]
    pub removal_proposal : Account<'info,ElectAuthority>,

    #[account(
        seeds =[
            CHALLENGEAUTHORITY,
            &proposal.proposal_id.to_le_bytes(),
            property_system.key().as_ref(),
        ],
        bump = proposal.bump,
        constraint = proposal.status == ProposalStatus::Executed @ ErrorCode::ProposalNotExecuted
    )]
    pub proposal : Account<'info,ChallengeProposal>,


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


pub fn add_new_authority_as_trustee(
    ctx:Context<AddNewAuthorityAsTrustee>
)->Result<()>{

     add_new_authority(
        &mut *ctx.accounts.removal_proposal,
        &ctx.accounts.candidate.candidate,
    )?;



    

    Ok(())



}