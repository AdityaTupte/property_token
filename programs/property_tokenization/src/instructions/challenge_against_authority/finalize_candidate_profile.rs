use anchor_lang::{prelude::*};

use crate::{common::{CANDIDATE_PROFILE, CHALLENGEAUTHORITY, PROPERTY_SYSTEM_SEEDS, ProposalStatus}, errors::ErrorCode, state::{CandidateProfile, ChallengeProposal, PropertySystemAccount}};

#[derive(Accounts)]
pub struct FinalizeCandidateProfile<'info>{

    
    pub signer : Signer<'info>,

     #[account(
        seeds =[
            CHALLENGEAUTHORITY,
            &proposal.proposal_id.to_le_bytes(),
            property_system.key().as_ref(),
        ],
        bump = proposal.bump,
        constraint = proposal.status == ProposalStatus::Executed @ ErrorCode::ProposalNotExecuted
    )]
    pub proposal : Account<'info,ChallengeProposal>,

    #[account(
        seeds=[
            PROPERTY_SYSTEM_SEEDS,
            &property_system.property_system_id.to_le_bytes()
        ],
        bump = property_system.bump,
    )]
    pub property_system : Account<'info,PropertySystemAccount>,

   #[account(
        seeds=[
                CANDIDATE_PROFILE,
                candidate_profile.candidate.as_ref()
        ],
        bump = candidate_profile.bump,
        
    )]
    pub candidate_profile : Account<'info,CandidateProfile>,
}

pub fn finalize_candidate_profile(
    ctx:Context<FinalizeCandidateProfile>
)->Result<()>{

    let candidate = &mut ctx.accounts.candidate_profile;

    let proposal = &mut ctx.accounts.proposal;

    let index  = proposal.index;

    require!(proposal.against.len() > index as usize ,ErrorCode::AlreadyFinalized);

    require!(proposal.against.contains(&candidate.key()),ErrorCode::AuthorityNotFound);

    require!(
    proposal.against.get(index as usize) == Some(&candidate.key()),
    ErrorCode::ChangeCandidateFinalization
);

    if candidate.actions_history < proposal.guilty {

        candidate.actions_history = proposal.guilty;

        proposal.index += 1;
    }

    ////emit

    Ok(())
}