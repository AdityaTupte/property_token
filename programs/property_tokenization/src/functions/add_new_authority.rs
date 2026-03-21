use anchor_lang::prelude::*;

use crate::{constant::{AuthorityGovernance,}, errors::ErrorCode, };


pub fn add_new_authority<T:AuthorityGovernance,>(
    item:&mut T,
    authority_candidate_key : &Pubkey,
)->Result<()>{


let current_time = Clock::get()?.unix_timestamp;

require!(
        current_time > *item.voting_for_authority_deadline() &&
        current_time <= *item.add_new_authority_deadline(),
        ErrorCode::AuthorityAddDeadline
 );

require!(!(*item.new_authority()).contains(authority_candidate_key),ErrorCode::DuplicateAuthority);

require!(item.new_authority().len() < item.authority_to_resign().len(), ErrorCode::VotingLimitReached);

(*item.new_authority()).push(*authority_candidate_key);
 

Ok(())
}