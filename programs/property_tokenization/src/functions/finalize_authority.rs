
use anchor_lang::prelude::*;

use crate::{common::ProposalStatus, constant::{AuthorityGovernance, AuthorityRegistry}, errors::ErrorCode, state::Resignation};


pub fn finalize_authority<T:AuthorityRegistry,U:AuthorityGovernance>(
    registry:&mut T,
    proposal: &mut U,
    resignation: &mut Resignation,
)->Result<()>{

    require!( 
        Clock::get()?.unix_timestamp >
        *proposal.challenge_new_authority_deadline(),
        ErrorCode::ChallegeDeadlineNotExpired
    );

    require!(
        proposal.new_authority().len() == proposal.authority_to_resign().len(),
        ErrorCode::InvalidAuthorityMapping
    );

    let new_auth = proposal.new_authority().clone();
let resign_auth = proposal.authority_to_resign().clone();

    for (to_remove, to_add) in resign_auth
        .iter()
        .zip(new_auth.iter())
    {
        let index = registry
            .registry()
            .iter()
            .position(|x| x == to_remove)
            .ok_or(ErrorCode::AuthorityNotFound)?;

        require!(
            !registry.registry().contains(to_add),
            ErrorCode::DuplicateAuthority
        );

        registry.registry() [index] = *to_add;
        
    }

    *proposal.proposal_status() = ProposalStatus::Executed ;

    *proposal.is_finalize() = true;

    
    resignation.status = ProposalStatus::Executed;


    Ok(())


}