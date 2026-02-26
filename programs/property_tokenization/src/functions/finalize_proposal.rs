use anchor_lang::prelude::*;

use crate::{constant::{Governance, ProposalStatus}, errors::ErrorCode};

pub fn finalize<T:Governance>(
    item:&mut T
)->Result<()>{

    let current_time= Clock::get()?.unix_timestamp;

    require!(current_time > *item.end_time(),ErrorCode::VotingStillActive);

    require!(
    *item.proposal_status() == ProposalStatus::Active,ErrorCode::AlreadyFinalized);

    let current_threshold = item.votes_for()
    .checked_add(*item.votes_against())
    .ok_or(ErrorCode::MathOverflow)?;

    if current_threshold >= *item.vote_threshold() {

        if *item.votes_for() > *item.votes_against() {
            
            *item.proposal_status() = ProposalStatus::Passed;

        }

        else {
            *item.proposal_status() = ProposalStatus::Failed;

        }

    }
    else {
        *item.proposal_status() = ProposalStatus::Rejected;
    }
    
    Ok(())
}