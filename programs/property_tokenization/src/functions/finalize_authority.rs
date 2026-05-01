
use anchor_lang::prelude::*;

use crate::{common::ProposalStatus, constant::{AuthorityGovernance,}, errors::ErrorCode, state::Resignation,};


pub fn finalize_authority<U:AuthorityGovernance>(
    proposal: &mut U,
    resignation : &mut Account<Resignation>,
)->Result<()>{

    require!( 
        Clock::get()?.unix_timestamp >
        *proposal.challenge_new_authority_deadline(),
        ErrorCode::ChallegeDeadlineNotExpired
    );

    

    // for (to_remove, to_add) in resign_auth
    //     .iter()
    //     .zip(new_auth.iter())
    // {
    //     let index = registry
    //         .registry()
    //         .iter()
    //         .position(|x| x == to_remove)
    //         .ok_or(ErrorCode::AuthorityNotFound)?;

    //     require!(
    //         !registry.registry().contains(to_add),
    //         ErrorCode::DuplicateAuthority
    //     );

    //     registry.registry() [index] = *to_add;
        
    // }


    *proposal.total_authority_to_resign() = proposal.total_authority_to_resign()
                                                    .checked_sub(1)
                                                    .ok_or(ErrorCode::MathOverflow)?;

    if *proposal.total_authority_to_resign() ==0  {
        *proposal.proposal_status() = ProposalStatus::Passed ;

    *proposal.is_finalize() = true; 
    }

     resignation.status = ProposalStatus::Executed;

     resignation.time = Clock::get()?.unix_timestamp;
   


    Ok(())


}