use anchor_lang::prelude::*;

use crate::{common::{CHALLENGEAUTHORITY, COUNTRY_SEED, PROPERTY_SYSTEM_SEEDS, ProposalStatus, ReasonType, STATE_SEEDS},errors::ErrorCode, state::{ChallengeProposal, Country, PropertySystemAccount, State}};



#[derive(Accounts)]

pub struct OutComeOFProposal<'info>{

    #[account(
        constraint = state.authorities.contains(&authority.key()) @ ErrorCode::NotAuthorized
    )]
    pub authority :Signer<'info>,

    #[account(
        seeds=[
            STATE_SEEDS,
            &state.state_id.to_le_bytes(),
            country.key().as_ref(),
        ],
        bump = state.bump
    )]
    pub state:Account<'info,State>,

  
    #[account(
        seeds=[
            COUNTRY_SEED,
            &country.country_id.to_le_bytes(),
        ],
        bump = country.bump
    )]
    pub country:Account<'info,Country>,

     #[account(
        seeds =[
            CHALLENGEAUTHORITY,
            &proposal.proposal_id.to_le_bytes(),
            property_system.key().as_ref(),
        ],
        bump = proposal.bump,
        constraint = proposal.status == ProposalStatus::Passed @ ErrorCode::ProposalNotPassed
    )]
    pub proposal : Account<'info,ChallengeProposal>,


    #[account(
            seeds=[
            PROPERTY_SYSTEM_SEEDS,
            &property_system.property_system_id.to_le_bytes()
        ],
        bump = property_system.bump,
        
    )]
    pub property_system: Account<'info,PropertySystemAccount>, 
}


pub fn outcome_of_proposal(
    ctx:Context<OutComeOFProposal>,
    outcome:ReasonType,
)->Result<()>{

    let proposal = &mut ctx.accounts.proposal;

    proposal.guilty = outcome;

    proposal.status = ProposalStatus::Executed;

Ok(())

}