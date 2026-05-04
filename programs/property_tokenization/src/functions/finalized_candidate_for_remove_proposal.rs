use anchor_lang::prelude::*;

use crate::{common::{AuthorityType}, constant::{AuthorityGovernance, AuthorityRegistry}, errors::ErrorCode, state::{AuthorityCandidate, CandidateProfile,}};



pub fn finalized_candidate_for_remove_proposal<T:AuthorityGovernance,U:AuthorityRegistry>(
    item:&mut T,
    _registry:&mut U,
    authority_candidate : &mut Account<AuthorityCandidate>,
    candidate_profile : &mut Account<CandidateProfile>,
)->Result<()>{

    require!(
        *item.is_finalize() &&
        !authority_candidate.is_finalized ,
        ErrorCode::AlreadyFinalized
    );

    // require!(
    //     authority_candidate.candidate == candidate_profile.candidate &&
    //     item.new_authority().contains(&authority_candidate.candidate),
    //     ErrorCode::NotAuthorized,
    // );


    // authority_candidate.selected = true;

    // authority_candidate.selected_time = Clock::get()?.unix_timestamp;




    authority_candidate.is_finalized = true;


    if *item.proposal_type() == AuthorityType::TRUSTEE {

        candidate_profile.total_selected_as_trustee += 1; 

    }
    
    else {
        candidate_profile.total_selected_as_arbitrar += 1;    
    }

    authority_candidate.is_finalized =true ;
    

    Ok(())
}