use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, };

use crate::{common::{PROPERTY_SYSTEM_SEEDS, PROPOSE_THRESHOLD, ProposalStatus, RT_CHG_PROPOSAL_SEEDS}, errors::ErrorCode, state::{NEWTHRESHOLDPROPOSAL, PropertySystemAccount, RTChgProposal}};



#[derive(Accounts)]
pub struct ProposeNewThreshold<'info>{

    #[account(
        mut,
    )]
    pub signer : Signer<'info>,

    #[account(
        constraint = property_system.governance_mint == mint.key() @ ErrorCode::GovernanceTokenInvalid
    )]
    pub mint : InterfaceAccount<'info,Mint>,

    #[account(
        associated_token::mint = mint ,
        associated_token::authority = signer
    )]
    pub ata: InterfaceAccount<'info,TokenAccount>,
    

    #[account(
        seeds=[
            RT_CHG_PROPOSAL_SEEDS,
            &proposal.proposal_id.to_le_bytes(),
            property_system.key().as_ref()
        ],
        bump = proposal.bump,
        constraint = proposal.status  == ProposalStatus::Passed @ ErrorCode::ProposalNotPassed,
    )]
    pub proposal: Account<'info,RTChgProposal>,

    #[account(
        seeds=[
            PROPERTY_SYSTEM_SEEDS,
            &property_system.property_system_id.to_le_bytes(),
        ],
        bump = property_system.bump,
    )]
    pub property_system:Account<'info,PropertySystemAccount>,

    #[account(
        init,
        payer = signer,
        seeds=[
            PROPOSE_THRESHOLD,
            proposal.key().as_ref(),
            signer.key().as_ref()
        ],
        bump,
        space = 8 + NEWTHRESHOLDPROPOSAL::SIZE 
    )]
    pub new_threshold : Account<'info,NEWTHRESHOLDPROPOSAL>,

    pub system_program : Program<'info,System>,

}

pub fn propose_new_threshold(
            ctx:Context<ProposeNewThreshold>,
            
            new_trustee_salary_threshold : u8,
            
            new_arbitrator_salary_threshold : u8,

            new_dividend_threshold: u8,

            new_reinvestment_threshold : u8,

            new_safety_threshold : u8,
)->Result<()>{

    let new_threshold = &mut ctx.accounts.new_threshold;

    require!(ctx.accounts.ata.amount > 0 ,ErrorCode::TokenAreZero);

    require!(
        new_trustee_salary_threshold +
        new_arbitrator_salary_threshold +
        new_dividend_threshold +
        new_reinvestment_threshold +
        new_safety_threshold == 100 ,
        ErrorCode::InvalidThreshold
    );


    let current_time = Clock::get()?.unix_timestamp;

    require!(
        current_time < ctx.accounts.proposal.threshold_submission_deadline,
        ErrorCode::ThresholdSubmissionEnd
    );

    new_threshold.property_system = ctx.accounts.property_system.key();

    new_threshold.proposal = ctx.accounts.proposal.key();

    new_threshold.signer = ctx.accounts.signer.key();

    new_threshold.new_trustee_salary_threshold = new_trustee_salary_threshold;

    new_threshold.new_arbitrator_salary_threshold = new_arbitrator_salary_threshold;

    new_threshold.new_dividend_threshold = new_dividend_threshold;

    new_threshold.new_reinvestment_threshold = new_reinvestment_threshold;

    new_threshold.new_safety_threshold = new_safety_threshold;

    new_threshold.bump = ctx.bumps.new_threshold;

    Ok(())
}