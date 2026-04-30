use anchor_lang::prelude::*;

use crate::{common::ProposalStatus, constant::AuthorityGovernance, errors::ErrorCode};


   pub fn submit_authority<T:AuthorityGovernance>(
        item: &mut T,
        merkle_root : [u8;32],
        candidate_submission_deadline: u8,
        voting_for_authority_deadline : u8,
        add_new_authority_deadline : u8,
        challenge_new_authority_deadline : u8,
    )->Result<()>{

    require!(
    candidate_submission_deadline > 0 && candidate_submission_deadline < 30 &&
    voting_for_authority_deadline > 0 && voting_for_authority_deadline < 30 &&
    add_new_authority_deadline > 0 && add_new_authority_deadline < 30 &&
    challenge_new_authority_deadline > 0 && challenge_new_authority_deadline < 30,
    ErrorCode::DeadlineIssue,
);

    let current_time = Clock::get()?.unix_timestamp;
    
    *item.merkle_root() = merkle_root;

    *item.candidate_submission_deadline() = current_time
                                                .checked_add(
                                                    (candidate_submission_deadline as i64)
                                                    .checked_mul((24*60*60) as i64)
                                                    .ok_or(ErrorCode::MathOverflow)?
                                                ).ok_or(ErrorCode::MathOverflow)?;

    
    *item.voting_for_authority_deadline() = item.candidate_submission_deadline()
                                                .checked_add(
                                                    (voting_for_authority_deadline as i64)
                                                    .checked_mul((24*60*60) as i64)
                                                    .ok_or(ErrorCode::MathOverflow)?
                                                ).ok_or(ErrorCode::MathOverflow)?;

    *item.add_new_authority_deadline() =    item.voting_for_authority_deadline()
                                                .checked_add(
                                                    (add_new_authority_deadline as i64)
                                                    .checked_mul((24*60*60) as i64)
                                                    .ok_or(ErrorCode::MathOverflow)?
                                                ).ok_or(ErrorCode::MathOverflow)?;
 
    *item.challenge_new_authority_deadline() = item.add_new_authority_deadline()
                                                .checked_add(
                                                    (challenge_new_authority_deadline as i64)
                                                    .checked_mul((24*60*60) as i64)
                                                    .ok_or(ErrorCode::MathOverflow)?
                                                ).ok_or(ErrorCode::MathOverflow)?;


    *item.proposal_status() = ProposalStatus::Active;

    *item.snapshot_submitted() = true;


    Ok(())


    }