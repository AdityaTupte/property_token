use anchor_lang::{ prelude::*};
use anchor_spl::associated_token::spl_associated_token_account::solana_program::keccak;

use crate::{constant::{ProposalStatus, TRANSFERPROPOSAL, VOTERRECIEPT}, errors::ErrorCode, functions::verify_proof, state::{PropertySystemAccount, TransferLandDetail, VoterReciept}};

#[derive(Accounts)]
pub struct Voting<'info>{

    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds=[
            TRANSFERPROPOSAL,
            &proposal.proposal_id.to_le_bytes(),
            property_system.key().as_ref(),
        ],
        bump = proposal.bump,
         constraint = proposal.snapshot_submitted @ ErrorCode::SnapshotNotSubmitted,
         constraint = proposal.proposal_status !=  ProposalStatus::Passed as u8 @ ErrorCode::ProposalAlreadyPassed,
         constraint = proposal.proposal_status == ProposalStatus::Active as u8 @ ErrorCode::ProposalNotActive
    )]

    pub proposal : Account<'info,TransferLandDetail>,


    #[account(
        constraint = proposal.source_property_system == property_system.key() @ ErrorCode::InvalidProposal
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

    let proposal = &mut ctx.accounts.proposal;
    
    let signer = &ctx.accounts.signer;

    let property_system = &ctx.accounts.property_system;
    let current_time = Clock::get()?.unix_timestamp;
    
    require!(current_time >= proposal.start_time  && current_time <= proposal.end_time , ErrorCode::VotingPeriodExpired);


    require!(voting_power <= proposal.total_voting_power, ErrorCode::VotingPowerInvalid);

    let leaf = keccak::hashv(&[
        signer.key().as_ref(),
        property_system.governance_mint.as_ref(),
        &voting_power.to_le_bytes(),
    ]).0;

    
    require!(
        verify_proof(leaf, &proof, proposal.merkle_root),
        ErrorCode::InvalidMerkleProof
    );

    if yes_or_no{

       proposal.votes_for = proposal
        .votes_for
        .checked_add(voting_power)
        .ok_or(ErrorCode::MathOverflow)?;

    } 

    else {
        proposal.votes_against = proposal
        .votes_against
        .checked_add(voting_power)
        .ok_or(ErrorCode::MathOverflow)?;
    }

   
    let receipt = &mut ctx.accounts.voter_receipt;
   
    receipt.proposal = proposal.key();
   
    receipt.voter = signer.key();

    if proposal.votes_for >= proposal.vote_required{

        proposal.proposal_status =  ProposalStatus::Passed as u8;       

        proposal.transfer_window = current_time;
    } 

    Ok(())



    }