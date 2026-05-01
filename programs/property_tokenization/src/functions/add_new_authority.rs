use anchor_lang::prelude::*;

use crate::{constant::AuthorityGovernance, errors::ErrorCode, state::{AuthorityCandidate, RankCounter, RankingAccount}, };


pub fn add_new_authority<T:AuthorityGovernance,>(
    item:&mut T,
    ranking_acc : &mut Account<RankingAccount>, 
    authority_candidate : &mut Account<AuthorityCandidate>,
    ranking:u8,
    proposal_key :Pubkey,
    bump:u8,
    counter : &mut Account<RankCounter>,
    // selection_receipt: &mut Account<AuthorityCandidateSelectionRecipt>,
    // selection_bump:u8,
)->Result<()>{


let current_time = Clock::get()?.unix_timestamp;

require!(current_time > *item.voting_for_authority_deadline() , ErrorCode::AuthorityAddingNotStarted);

require!(
        
        current_time <= *item.add_new_authority_deadline(),
        ErrorCode::AuthorityAddDeadline
 );

// require!(!(*item.new_authority()).contains(authority_candidate_key),ErrorCode::DuplicateAuthority);

// require!(item.new_authority().len() < item.authority_to_resign().len(), ErrorCode::VotingLimitReached);

require!( 
    ranking <= *item.total_authority_to_resign()  
    &&
    ranking >= 1 , ErrorCode::RankingNotBetween);


    require!(counter.count < *item.total_authority_to_resign() , ErrorCode::AuthorityLimitReached );

    counter.count +=1;

    authority_candidate.selected = true;

    authority_candidate.selected_time = Clock::get()?.unix_timestamp;

ranking_acc.candidate_key = authority_candidate.candidate;

ranking_acc.elect_proposal = proposal_key;

ranking_acc.rank = ranking;

ranking_acc.bump = bump;

// selection_receipt.candidate_key = authority_candidate.candidate;

// selection_receipt.proposal = proposal_key;

// selection_receipt.bump = selection_bump;

// (*item.new_authority()).push(*authority_candidate_key);
 

Ok(())
}