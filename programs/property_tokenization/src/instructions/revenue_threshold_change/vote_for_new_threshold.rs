use anchor_lang::{prelude::*};
use anchor_spl::{ token_interface::{Mint, TokenAccount}};


use crate::{common::{PROPERTY_SYSTEM_SEEDS, PROPOSE_THRESHOLD, ProposalStatus, RT_CHG_PROPOSAL_SEEDS, THRESHOLD_VOTE_RECEIPT}, errors::ErrorCode, state::{NEWTHRESHOLDPROPOSAL, PropertySystemAccount, RTChgProposal, ThresholdVoteReceipt}};


#[derive(Accounts)]

 pub struct VoteForNewThreshold<'info>{

    #[account(
        mut,
    )]
    pub signer :Signer<'info>,
    
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
            PROPOSE_THRESHOLD,
            proposal.key().as_ref(),
            new_threshold.signer.as_ref()
        ],
        bump = new_threshold.bump
    )]
    pub new_threshold: Account<'info,NEWTHRESHOLDPROPOSAL>,

    #[account(
        seeds=[
            RT_CHG_PROPOSAL_SEEDS,
            &proposal.proposal_id.to_le_bytes(),
            property_system.key().as_ref()
        ],
        bump = proposal.bump,
        constraint = proposal.status == ProposalStatus::Passed @ ErrorCode::ProposalNotPassed
    )]
    pub proposal : Account<'info,RTChgProposal>,

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
            THRESHOLD_VOTE_RECEIPT,
            signer.key().as_ref(),
            proposal.key().as_ref(),
        ],
        bump,
        space = 8 + ThresholdVoteReceipt::SIZE 
    )]
    pub new_threshold_vote_receipt : Account<'info,ThresholdVoteReceipt>,
    
    pub system_program : Program<'info,System>,

}


pub fn vote_for_new_threshold(ctx:Context<VoteForNewThreshold>)->Result<()>{
    
    require!(ctx.accounts.ata.amount > 0 ,ErrorCode::TokenAreZero);

    let current_time = Clock::get()?.unix_timestamp ;

    let proposal = &mut ctx.accounts.proposal;

    require!(
        current_time > proposal.threshold_submission_deadline &&
        current_time < proposal.voting_for_threshold_deadline,
        ErrorCode::VotingPeriodExpired 
    );

    let new_threshold_option = &mut ctx.accounts.new_threshold;

    new_threshold_option.vote_gained += ctx.accounts.ata.amount;

    let receipt = &mut ctx.accounts.new_threshold_vote_receipt;

    receipt.thresholdvoted = new_threshold_option.key();

    receipt.bump = ctx.bumps.new_threshold_vote_receipt;

    Ok(())

}
