use anchor_lang::prelude::*;

use crate::{common::{AUTHORITY_CANDIDATE, ELECT_ARBITRAR, PROPERTY_SYSTEM_SEEDS, ProposalStatus, RANKINGACCOUNT}, errors::ErrorCode, functions::challenge_authority, state::{AuthorityCandidate, ElectAuthority, PropertySystemAccount, RankingAccount}};


#[derive(Accounts)]
#[instruction(proposal_id:u64,challenge_from_key:Pubkey,challenge_to_key:Pubkey,ranking:u8,property_system_id:u64)]
pub struct ChallengeNewArbitrar<'info>{

    #[account(mut)]
    pub signer:Signer<'info>,

    #[account(
        mut,
        seeds=[
            ELECT_ARBITRAR,
            property_system.key().as_ref(),
            &proposal_id.to_le_bytes()
        ],
        bump = proposal.bump,
        constraint = proposal.snapshot_submitted @ ErrorCode::SnapshotNotSubmitted,
        constraint = proposal.status == ProposalStatus::Active @ ErrorCode::ProposalNotActive
    )]
    pub proposal : Account<'info,ElectAuthority>,

    #[account(
        seeds=[
            PROPERTY_SYSTEM_SEEDS,
            &property_system_id.to_le_bytes(),
        ],
        bump = property_system.bump
    )]
    pub property_system:Account<'info,PropertySystemAccount>,

    #[account(
        mut,
        seeds=[
            AUTHORITY_CANDIDATE,
            property_system.key().as_ref(), 
            proposal.key().as_ref(),
            challenge_from_key.as_ref(), 
        ],
        bump = challenge_from.bump
    )]
    pub challenge_from: Account<'info,AuthorityCandidate>,
    
    #[account(
        seeds=[
            AUTHORITY_CANDIDATE,
            property_system.key().as_ref(), 
            proposal.key().as_ref(),
            challenge_to_key.as_ref(),
        ],
        bump = challenge_to.bump,
        constraint = challenge_to.selected @ ErrorCode::AuthorityNotSelected,
        constraint = challenge_from.key() != challenge_to.key() @ ErrorCode::DuplicateAuthority
    )]
    pub challenge_to: Account<'info,AuthorityCandidate>,


    #[account(
        mut,
        seeds=[
            RANKINGACCOUNT,
            &ranking.to_le_bytes(),
            proposal.key().as_ref(),
            property_system.key().as_ref()
        ],
        bump = ranking_acc.bump,
    )]
    pub ranking_acc : Account<'info,RankingAccount>,



}


pub fn challenge_new_arbitrar(
    ctx:Context<ChallengeNewArbitrar>,
    _proposal_id:u64,
    _challenge_from_key:Pubkey,
    _challenge_to_key:Pubkey,
    _ranking:u8,
    _property_system_id:u64,
)->Result<()>{

    challenge_authority(
        &mut *ctx.accounts.proposal,
        &mut ctx.accounts.challenge_from, 
        &mut ctx.accounts.challenge_to, 
       &mut ctx.accounts.ranking_acc,
    )?;

    Ok(())


}