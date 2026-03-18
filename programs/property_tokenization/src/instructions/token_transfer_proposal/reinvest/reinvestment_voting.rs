use anchor_lang::prelude::*;

use crate::{common::{ProposalStatus, USEREINVESTMENTOKEN, VOTERRECIEPT}, errors::ErrorCode, functions::voting, state::{PropertySystemAccount, UseReinvestmentProposal,  VoterReciept}};



#[derive(Accounts)]
pub struct Voting<'info>{

    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds=[
            USEREINVESTMENTOKEN,
            &proposal.proposal_id.to_le_bytes(),
            property_system.key().as_ref(),
        ],
        bump = proposal.bump,
         constraint = proposal.snapshot_submitted @ ErrorCode::SnapshotNotSubmitted,
         constraint = proposal.status !=  ProposalStatus::Passed  @ ErrorCode::ProposalAlreadyPassed,
         constraint = proposal.status == ProposalStatus::Active @ ErrorCode::ProposalNotActive
    )]

    pub proposal : Account<'info,UseReinvestmentProposal>,

    #[account(
        constraint = proposal.property_system == property_system.key() @ ErrorCode::InvalidProposal
    )]
    pub property_system: Account<'info,PropertySystemAccount>,
        
    #[account(
        init,
        payer = signer,
        seeds=[
            VOTERRECIEPT,
            proposal.key().as_ref(),
            signer.key().as_ref(),
        ],
        bump,
        space =  8 + VoterReciept::SIZE
    )]

    pub voter_receipt : Account<'info,VoterReciept>,

    pub system_program : Program<'info,System>,

}

pub fn vote(
        ctx:Context<Voting>,
        proof: Vec<[u8; 32]>,
        voting_power : u64,
        yes_or_no : bool,
    )->Result<()>{
    
    let proposal_key  =  ctx.accounts.proposal.key();

    let proposal= &mut *ctx.accounts.proposal;
    
    let signer = &ctx.accounts.signer;

    let property_system = & ctx.accounts.property_system;
    
    let receipt = &mut *ctx.accounts.voter_receipt;

    let recepit_bump = ctx.bumps.voter_receipt;
    
    let current_time = Clock::get()?.unix_timestamp;
    
    require!(current_time >= proposal.start_time  && current_time <= proposal.end_time , ErrorCode::VotingPeriodExpired);

    require!(voting_power > 0, ErrorCode::VotingPowerInvalid);

    voting(
        proposal,
        receipt,
        proof,
        voting_power,
        yes_or_no,
        signer.key(),
        property_system.governance_mint,
        proposal_key,
        recepit_bump,
        USEREINVESTMENTOKEN
    )?;


    Ok(())



    }