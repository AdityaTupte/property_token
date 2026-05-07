use anchor_lang::prelude::*;

use crate::{common::{PROPERTY_SYSTEM_SEEDS, PROPOSE_THRESHOLD, ProposalStatus, RT_CHG_PROPOSAL_SEEDS, THRESHOLD}, errors::ErrorCode, state::{NEWTHRESHOLDPROPOSAL, PropertySystemAccount, RTChgProposal, Threshold}};

#[derive(Accounts)]
#[instruction(property_system_id:u64,new_threshold_signer:Pubkey)]
pub struct FinalizeNewThreshold<'info>{

    #[account()]
    pub signer : Signer<'info>,

     #[account(
        seeds=[
            RT_CHG_PROPOSAL_SEEDS,
            property_system.key().as_ref(),
            &proposal.proposal_id.to_le_bytes(),
            
        ],
        bump,
        constraint = proposal.status  == ProposalStatus::Passed @ ErrorCode::ProposalNotPassed,
        constraint = proposal.new_threshold == new_threshold.key() @ ErrorCode::InvalidProposal
    )]
    pub proposal : Account<'info,RTChgProposal>, 

     #[account(
        seeds=[
            PROPERTY_SYSTEM_SEEDS,
            &property_system_id.to_le_bytes(),
        ],
        bump = property_system.bump,
    )]
    pub property_system:Account<'info,PropertySystemAccount>,

    #[account(
        mut,
        seeds = [
            THRESHOLD,
            property_system.key().as_ref()
        ],
        bump= existing_threshold.bump,
    )]

    pub existing_threshold : Account<'info,Threshold>,

    #[account(
        seeds=[
            PROPOSE_THRESHOLD,
            proposal.key().as_ref(),
            new_threshold_signer.as_ref()
        ],
        bump=new_threshold.bump,
    )]
    pub new_threshold : Account<'info,NEWTHRESHOLDPROPOSAL>,
}


pub fn finalize_new_threshold(
    ctx:Context<FinalizeNewThreshold>
)->Result<()>{

    let current_time = Clock::get()?.unix_timestamp;

    let existing_thrshold = &mut ctx.accounts.existing_threshold;

    let new_threshold = &ctx.accounts.new_threshold;

    let proposal = &mut ctx.accounts.proposal;

    require!(current_time > proposal.challenge_new_threshold_deadline,ErrorCode::ChallengeeDeadlineNotExpired);

    existing_thrshold.trustee_salary_threshold = new_threshold.new_trustee_salary_threshold;

    existing_thrshold.arbitrator_salary_threshold = new_threshold.new_arbitrator_salary_threshold; 

    existing_thrshold.dividend_threshold = new_threshold.new_dividend_threshold;

    existing_thrshold.reinvestment_threshold = new_threshold.new_reinvestment_threshold;

    existing_thrshold.safety_threshold = new_threshold.new_safety_threshold;

    proposal.status = ProposalStatus::Executed;

    

    Ok(())
}