use anchor_lang::prelude::*;

use crate::{common::{PROPERTY_SYSTEM_SEEDS, PROPOSE_THRESHOLD, ProposalStatus, RT_CHG_PROPOSAL_SEEDS}, errors::ErrorCode, state::{NEWTHRESHOLDPROPOSAL, PropertySystemAccount, RTChgProposal}};


#[derive(Accounts)]
pub struct ChallengeNewThreshold<'info>{

    #[account()]
    pub signer: Signer<'info>,

    #[account(
        seeds=[
            PROPERTY_SYSTEM_SEEDS,
            &property_system.property_system_id.to_le_bytes(),
        ],
        bump = property_system.bump,
    )]
    pub property_system:Account<'info,PropertySystemAccount>,

    #[account(
        seeds=[
            RT_CHG_PROPOSAL_SEEDS,
            &proposal.proposal_id.to_le_bytes(),
            property_system.key().as_ref()
        ],
        bump,
        constraint = proposal.status  == ProposalStatus::Passed @ ErrorCode::ProposalNotPassed
    )]
    pub proposal : Account<'info,RTChgProposal>, 
    
    #[account(
        seeds=[
            PROPOSE_THRESHOLD,
            proposal.key().as_ref(),
            existing_new_threshold.signer.as_ref()
        ],
        bump=existing_new_threshold.bump,
    )]
    pub existing_new_threshold : Account<'info,NEWTHRESHOLDPROPOSAL>,

    #[account(
        seeds=[
            PROPOSE_THRESHOLD,
            proposal.key().as_ref(),
            challenge_new_threshold.signer.as_ref()
        ],
        bump=challenge_new_threshold.bump,
    )]
    pub challenge_new_threshold : Account<'info,NEWTHRESHOLDPROPOSAL>,

} 

pub fn challenge_new_threshold(ctx:Context<ChallengeNewThreshold>)->Result<()>{

    let current_time = Clock::get()?.unix_timestamp;

    let proposal = &mut ctx.accounts.proposal;

    let existing_threshold = & ctx.accounts.existing_new_threshold;

    let challenge_threshold = & ctx.accounts.challenge_new_threshold;


    require!(
        current_time > proposal.add_new_threshold_deadline &&
        current_time < proposal.challenge_new_threshold_deadline,
        ErrorCode::ChallegeDeadlineExpired
    );


    if challenge_threshold.vote_gained > existing_threshold.vote_gained  {
        
        proposal.new_threshold = challenge_threshold.key();

    }
    
    Ok(())


}