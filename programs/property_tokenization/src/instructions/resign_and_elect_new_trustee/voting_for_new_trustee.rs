use anchor_lang::{ prelude::*};

use crate::{common::{AUTHORITY_CANDIDATE, AUTHORITYVOTERECEIPT, AuthorityType, ELECT_TRUSTEE, PROPERTY_SYSTEM_SEEDS, ProposalStatus}, errors::ErrorCode, state::{AuthorityCandidate, AuthorityVoteReceipt, ElectAuthority, PropertySystemAccount}};


#[derive(Accounts)]

pub struct VotingForNewTrustee<'info>{

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
            ELECT_TRUSTEE,
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
    ctx:Context<VotingForNewTrustee>,
    proof: Vec<[u8; 32]>,
    voting_power : u64
)->Result<()>{

    let current_time = Clock::get()?.unix_timestamp;

    let proposal = &ctx.accounts.proposal;

    let authority_candidate = &mut ctx.accounts.authority_candidate;

    let receipt = &mut ctx.accounts.authority_vote_receipt;


    require!(
        current_time <= proposal.voting_for_authority_deadline &&
        current_time > proposal.candidate_submission_deadline,
        ErrorCode::AuthorityVotingDeadline
    );

    require!(authority_candidate.authority_type == AuthorityType::TRUSTEE,ErrorCode::AuthotityTypeNotMatched);

    //leaf check 


    if receipt.is_initialized {
        
        receipt.voter = ctx.accounts.signer.key();

        receipt.proposal = proposal.key();

        receipt.voting_power = voting_power;

        receipt.bump = ctx.bumps.authority_vote_receipt;

    }

    require!(proposal.authority_to_resign.len() > receipt.votes.len(),ErrorCode::VotingLimitReached);

    require!(!receipt.votes.contains(&authority_candidate.candidate),ErrorCode::DuplicateAuthority);

    authority_candidate.vote_gained = authority_candidate.vote_gained
                                .checked_add(voting_power)
                                .ok_or(ErrorCode::MathOverflow)?;
    

    receipt.votes.push(authority_candidate.candidate);



    Ok(())

}