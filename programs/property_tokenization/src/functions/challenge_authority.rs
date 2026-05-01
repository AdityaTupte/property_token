use anchor_lang::prelude::*;

use crate::{constant::AuthorityGovernance,errors::ErrorCode, state::{AuthorityCandidate, RankingAccount} };



pub fn challenge_authority<T:AuthorityGovernance>(
    item:&mut T,
    challenge_from : &mut Account<AuthorityCandidate>,
    challenge_to : &mut Account<AuthorityCandidate>,
    ranking_acc : &mut Account<RankingAccount>,

)->Result<()>{
    
    let current_time  = Clock::get()?.unix_timestamp;

    require!( 
        current_time > *item.add_new_authority_deadline() ,
        ErrorCode::ChallegeDeadlineNotStart
    );

    require!(
        current_time <= *item.challenge_new_authority_deadline(),
        ErrorCode::ChallegeDeadlineExpired
    );

    
    require!(ranking_acc.rank == *item.total_authority_to_resign(), ErrorCode::NotRankChangeRequired);
  

    require_gt!(
    challenge_from.vote_gained,
    challenge_to.vote_gained,
    ErrorCode::VoteGainedLess
    );

    challenge_from.selected = true;

    challenge_to.selected = false;

    challenge_from.selected_time = Clock::get()?.unix_timestamp;
    
    ranking_acc.candidate_key = challenge_from.candidate;

Ok(())

}