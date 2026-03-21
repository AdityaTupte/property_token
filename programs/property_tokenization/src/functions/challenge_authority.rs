use anchor_lang::prelude::*;

use crate::{constant::AuthorityGovernance,errors::ErrorCode, state::AuthorityCandidate };



pub fn challenge_authority<T:AuthorityGovernance>(
    item:&mut T,
    challenge_from : &AuthorityCandidate,
    challenge_from_key:&Pubkey,
    challenge_to : &AuthorityCandidate,
    challenge_to_key: &Pubkey,
)->Result<()>{
    
    let current_time  = Clock::get()?.unix_timestamp;
    
    require!(
    challenge_from_key != challenge_to_key,
    ErrorCode::InvalidChallenge);

    require!(
        current_time > *item.add_new_authority_deadline() &&
        current_time <= *item.challenge_new_authority_deadline(),
        ErrorCode::ChallegeDeadlineExpired
    );

     require!(!item.new_authority().contains(challenge_from_key),ErrorCode::DuplicateAuthority);

  

    require_gt!(
    challenge_from.vote_gained,
    challenge_to.vote_gained,
    ErrorCode::VoteGainedLess
    );

     let index = item
    .new_authority()
    .iter()
    .position(|x| x == challenge_to_key)
    .ok_or(ErrorCode::ChallengeToNotInNewAuthority)?;

    item.new_authority().swap_remove(index);

    item.new_authority().push(*challenge_from_key);
    

Ok(())

}