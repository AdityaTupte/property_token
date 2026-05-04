use anchor_lang::prelude::*;

use crate::{common::{AUTHORITY_CANDIDATE,  PROPERTY_SYSTEM_SEEDS, ProposalStatus, RANKINGACCOUNT, }, errors::ErrorCode, functions::challenge_authority, state::{AuthorityCandidate,  ElectAuthority, PropertySystemAccount, RankingAccount}};


#[derive(Accounts)]
#[instruction(proposal_key:Pubkey,property_system_id:u64,challenge_from_key:Pubkey,challenge_to_key:Pubkey,ranking:u8)]
pub struct RemovalProposalChallengeNewAuthority<'info>{

    
    pub signer:Signer<'info>,


   #[account(
        mut,
        // seeds=[
        //     REMOVEAUTHORITY,
        //     property_system.key().as_ref(),
        //     proposal_key.as_ref()
        // ],
        // bump = removal_proposal.bump,
        constraint = removal_proposal.snapshot_submitted @ ErrorCode::SnapshotNotSubmitted,
        constraint = removal_proposal.status == ProposalStatus::Approved @ ErrorCode::ProposalNotApproved,
        // constraint = removal_proposal.authority_type == AuthorityType::TRUSTEE
    )]
    pub removal_proposal : Account<'info,ElectAuthority>,

    #[account(
        seeds=[
            PROPERTY_SYSTEM_SEEDS,
            &property_system_id.to_le_bytes(),
        ],
        bump = property_system.bump
    )]
    pub property_system:Account<'info,PropertySystemAccount>,

    #[account(
        seeds=[
            AUTHORITY_CANDIDATE,
            property_system.key().as_ref(), 
            removal_proposal.key().as_ref(),
            challenge_from_key.as_ref()
        ],
        bump = challenge_from.bump
    )]
    pub challenge_from: Account<'info,AuthorityCandidate>,
    
    #[account(
        seeds=[
            AUTHORITY_CANDIDATE,
            property_system.key().as_ref(), 
            removal_proposal.key().as_ref(),
            challenge_to_key.as_ref()
        ],
        bump = challenge_to.bump
    )]
    pub challenge_to: Account<'info,AuthorityCandidate>,

    #[account(
        mut,
        seeds=[
            RANKINGACCOUNT,
            &ranking.to_le_bytes(),
            removal_proposal.key().as_ref(),
            property_system.key().as_ref()
        ],
        bump = ranking_acc.bump,
    )]
    pub ranking_acc : Account<'info,RankingAccount>,

}


pub fn challenge_new_authority(
    ctx:Context<RemovalProposalChallengeNewAuthority>,
    _proposal_key:Pubkey,_property_system_id:u64,_challenge_from_key:Pubkey,_challenge_to_key:Pubkey,_ranking:u8
)->Result<()>{

   challenge_authority(
        &mut *ctx.accounts.removal_proposal,
        &mut ctx.accounts.challenge_from, 
        &mut ctx.accounts.challenge_to, 
       &mut ctx.accounts.ranking_acc,
    )?;

    Ok(())


}