use anchor_lang::prelude::*;

use crate::{constant::{Governance, ProposalStatus}, errors::ErrorCode};

pub fn delete_proposal<T:Governance>(
    item:&mut T
) ->Result<()>{

    let current_time = Clock::get()?.unix_timestamp;

    require!(
    (
        *item.proposal_status() == ProposalStatus::Draft
        && current_time < *item.start_time()
    )
    ||
    (
        current_time > *item.end_time()
        && *item.proposal_status() == ProposalStatus::Rejected
    ),
    ErrorCode::DeleteNotAllowed
);

    Ok(())
}
