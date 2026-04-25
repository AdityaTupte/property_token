use anchor_lang::{ prelude::*};

use crate::{ common::{BUYPROPERTY, PROPERTY_SYSTEM_SEEDS, ProposalStatus, VOTERRECIEPT}, errors::ErrorCode, functions::voting, state::{PropertyBuyProposal, PropertySystemAccount, VoterReciept}};

#[derive(Accounts)]
#[instruction(proposal_id:u64,property_system_id:u64)]
pub struct BuyProposalVoting<'info>{

    #[account(mut)]
    pub signer: Signer<'info>,

     #[account(
        mut,
        seeds=[
            BUYPROPERTY,
            property_system.key().as_ref(),
            &proposal_id.to_le_bytes(),
        ],
        bump = proposal.bump,
         constraint = proposal.snapshot_submitted @ ErrorCode::SnapshotNotSubmitted,
         constraint = proposal.status !=  ProposalStatus::Passed  @ ErrorCode::ProposalAlreadyPassed,
         constraint = proposal.status == ProposalStatus::Active @ ErrorCode::ProposalNotActive
    )]

    pub proposal : Account<'info,PropertyBuyProposal>,

     #[account(
        seeds=[
            PROPERTY_SYSTEM_SEEDS,
            &property_system_id.to_le_bytes()
        ],
        bump= property_system.bump,

        constraint = proposal.buyer == property_system.key() @ ErrorCode::InvalidProposal
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
        ctx:Context<BuyProposalVoting>,
        _proposal_id : u64,
        _property_system_id : u64,
        proof: Vec<[u8; 32]>,
        voting_power : u64,
        yes_or_no : bool,
    )->Result<()>{
    
    let proposal_key  =  ctx.accounts.proposal.key();

    let proposal= &mut *ctx.accounts.proposal;
    
    let signer = &ctx.accounts.signer;

    let property_system = &ctx.accounts.property_system;
    
    let receipt = &mut *ctx.accounts.voter_receipt;

    let receipt_bump = ctx.bumps.voter_receipt;
    
    let current_time = Clock::get()?.unix_timestamp;
    
    require!(current_time >= proposal.start_time  , ErrorCode::VotingPeriodNotStarted);

    require!(current_time <= proposal.end_time ,ErrorCode::VotingPeriodExpired);

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
        receipt_bump,
        BUYPROPERTY
    )?;


    Ok(())



    }