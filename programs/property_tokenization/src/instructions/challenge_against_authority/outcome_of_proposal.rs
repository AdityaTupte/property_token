use anchor_lang::prelude::*;

use crate::{common::{CHALLENGEAUTHORITY,  HARDCODED_PUBKEY, PROPERTY_SYSTEM_SEEDS, ProposalStatus, ReasonType, },errors::ErrorCode, state::{ChallengeProposal,  PropertySystemAccount,}};



#[derive(Accounts)]
#[instruction(proposal_id : u64,property_system_id : u64)]
pub struct OutComeOFProposal<'info>{

    #[account(
        address = HARDCODED_PUBKEY,
    )]
    pub authority :Signer<'info>,

    // #[account(
    //     seeds=[
    //         STATE_AUTHORITY,
    //         country.key().as_ref(),
    //         authority.key().as_ref()
    //     ],
    //     bump =state_authority_receipt.bump
    // )]
    // pub state_authority_receipt : Account<'info,StateAuthority>,

    // #[account(
    //     seeds=[
    //         STATE_SEEDS,
    //         state_name.as_ref(),
    //         country.key().as_ref(),
    //     ],
    //     bump = state.bump
    // )]
    // pub state:Account<'info,State>,

  
    // #[account(
    //     seeds=[
    //         COUNTRY_SEED,
    //         country_name.as_ref()
    //     ],
    //     bump = country.bump
    // )]
    // pub country:Account<'info,Country>,

     #[account(
        mut,
        seeds =[
            CHALLENGEAUTHORITY,
            property_system.key().as_ref(),
            &proposal_id.to_le_bytes(),
        ],
        bump = proposal.bump,
        constraint = proposal.status == ProposalStatus::Passed @ ErrorCode::ProposalNotPassed
    )]
    pub proposal : Account<'info,ChallengeProposal>,


    #[account(
            seeds=[
            PROPERTY_SYSTEM_SEEDS,
            &property_system_id.to_le_bytes()
        ],
        bump = property_system.bump,
        
    )]
    pub property_system: Account<'info,PropertySystemAccount>, 
}


pub fn outcome_of_proposal(
    ctx:Context<OutComeOFProposal>,
    _proposal_id : u64,
    _property_system_id : u64,
    outcome:ReasonType,
)->Result<()>{

    let proposal = &mut ctx.accounts.proposal;

    proposal.guilty = outcome;

    proposal.status = ProposalStatus::Executed;

    proposal.result_time = Clock::get()?.unix_timestamp;

    Ok(())

}