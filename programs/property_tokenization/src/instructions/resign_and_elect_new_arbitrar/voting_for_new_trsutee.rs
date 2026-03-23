use anchor_lang::{ prelude::*};

use crate::{common::{AUTHORITY_CANDIDATE, AUTHORITYVOTERECEIPT, ELECT_ARBITRAR, PROPERTY_SYSTEM_SEEDS, ProposalStatus}, errors::ErrorCode, functions::voting_for_authority, state::{AuthorityCandidate, AuthorityVoteReceipt, ElectAuthority, PropertySystemAccount}};


#[derive(Accounts)]

pub struct VotingForNewArbitrar<'info>{

    #[account(
        mut,
    )]
    pub signer: Signer<'info>,

    #[account(
        seeds =[
            AUTHORITY_CANDIDATE,
            authority_candidate.candidate.as_ref(),
            proposal.key().as_ref(),
            property_system.key().as_ref()
        ],
        bump = authority_candidate.bump
    )]
    pub authority_candidate : Account<'info,AuthorityCandidate>,

    #[account(
        seeds=[
            ELECT_ARBITRAR,
            &proposal.proposal_id.to_le_bytes(),
            property_system.key().as_ref()
        ],
        bump= proposal.bump,
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
    pub property_system : Account<'info,PropertySystemAccount>,

    #[account(
        init_if_needed,
        payer = signer,
        seeds=[
            AUTHORITYVOTERECEIPT,
            signer.key().as_ref(),
            proposal.key().as_ref(),
        ],
        bump,
        space = 8 + AuthorityVoteReceipt::SIZE
    )]
    pub authority_vote_receipt: Account<'info, AuthorityVoteReceipt>,

    pub system_program :Program<'info,System>,

}


pub fn voting_for_new_trustee(
    ctx:Context<VotingForNewArbitrar>,
    proof: Vec<[u8; 32]>,
    voting_power : u64,
)->Result<()>{
    
    voting_for_authority(
        ctx.accounts.proposal.key(),
        &mut ctx.accounts.authority_candidate, 
        &mut ctx.accounts.authority_vote_receipt, 
        ctx.accounts.signer.key(), 
        &mut *ctx.accounts.proposal,
        proof, 
        ctx.bumps.authority_vote_receipt, 
        voting_power,
        &ctx.accounts.property_system.governance_mint,
    )?;

    


    Ok(())

}