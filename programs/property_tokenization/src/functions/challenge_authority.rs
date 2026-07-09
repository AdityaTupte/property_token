use anchor_lang::prelude::*;

use crate::{errors::ErrorCode, events::AuthorityRevise, state::{AuthorityCandidate, RankingAccount}, traits::AuthorityGovernance };



pub fn challenge_authority<T:AuthorityGovernance>(
    item:&mut T,
    challenge_from : &mut Account<AuthorityCandidate>,
    challenge_to : &mut Account<AuthorityCandidate>,
    ranking_acc : &mut Account<RankingAccount>,
    proposal_key: Pubkey
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

   emit!(
    AuthorityRevise{
        proposal_key:proposal_key,
        new_authority:challenge_from.candidate,
        old_authority:challenge_to.candidate,
    }
   );
Ok(())

}