use anchor_lang::{ prelude::*};

use crate::{common::{AUTHORITY_CANDIDATE, AUTHORITYVOTERECEIPT, ELECT_ARBITRAR, PROPERTY_SYSTEM_SEEDS, ProposalStatus}, errors::ErrorCode, functions::voting_for_authority, state::{AuthorityCandidate, AuthorityVoteReceipt, ElectAuthority, PropertySystemAccount}};


#[derive(Accounts)]
#[instruction(proposal_id:u64, property_system_id : u64,candidate_key:Pubkey)]
pub struct VotingForNewArbitrar<'info>{

    #[account(
        mut,
    )]
    pub signer: Signer<'info>,


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
            ELECT_ARBITRAR,
            property_system.key().as_ref(),
            &proposal_id.to_le_bytes()
        ],
        bump= proposal.bump,
        constraint = proposal.snapshot_submitted @ ErrorCode::SnapshotNotSubmitted,
        constraint = proposal.status == ProposalStatus::Active @ ErrorCode::ProposalNotActive
    )]
    pub proposal : Account<'info,ElectAuthority>,

    #[account(
        mut,
        seeds =[
            AUTHORITY_CANDIDATE,
            property_system.key().as_ref(), 
            proposal.key().as_ref(),
            candidate_key.as_ref(),
        ],
        bump = authority_candidate.bump,
        constraint = authority_candidate.proposal == proposal.key()  @ ErrorCode::AuthorityNotMatchWithProposal
    )]
    pub authority_candidate : Account<'info,AuthorityCandidate>,

    #[account(
        init,
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


pub fn voting_for_new_arbitrar(
    ctx:Context<VotingForNewArbitrar>,
    _proposal_id:u64,_property_system_id:u64,_candidate_key:Pubkey,
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