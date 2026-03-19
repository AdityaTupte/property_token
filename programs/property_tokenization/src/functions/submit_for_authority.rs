use anchor_lang::prelude::*;

use crate::{constant::AuthorityGovernance, errors::ErrorCode};


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

    *item.merkle_root() = merkle_root;



    Ok(())


    }