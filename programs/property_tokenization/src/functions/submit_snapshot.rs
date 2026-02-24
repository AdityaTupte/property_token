use anchor_lang::prelude::*;

use crate::constant::{ProposalStatus, Snapshot};
use crate::errors::ErrorCode;

    pub fn submit<T:Snapshot>(
        item: &mut T,
        merkle_root : [u8;32],
        closing_days : u8,
    )->Result<()>{

    let current_time = Clock::get()?.unix_timestamp;

    let one_day: i64 = 24 * 60 * 60;

    let closing_time = one_day
            .checked_mul(closing_days as i64)
            .ok_or(ErrorCode::MathOverflow)?;

        let start_time = current_time
            .checked_add(one_day)
            .ok_or(ErrorCode::MathOverflow)?;

        let end_time = start_time
            .checked_add(closing_time)
            .ok_or(ErrorCode::MathOverflow)?;

        *item.start_time() = start_time;

        *item.end_time() = end_time;

        *item.merkle_root() = merkle_root;

        *item.snapshot_submitted() = true;

        let vote_required = item.total_voting_power()
                                            .checked_mul(65)
                                            .ok_or(ErrorCode::MathOverflow)?
                                            .checked_add(99)
                                            .ok_or(ErrorCode::MathOverflow)?
                                            / 100; 
        
        *item.vote_required() = vote_required;

        *item.proposal_status() = ProposalStatus::Active;



    Ok(())

    }