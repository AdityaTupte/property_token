use anchor_lang::prelude::*;

use crate::{common::{PROPERTY_SYSTEM_SEEDS, PROPOSE_THRESHOLD, ProposalStatus, RT_CHG_PROPOSAL_SEEDS, TRUSTEE_RECEIPT_SEEDS, TRUSTEEREGISTRYSEEDS}, errors::ErrorCode, state::{NEWTHRESHOLDPROPOSAL, PropertySystemAccount, RTChgProposal, TrusteeRecepit, TrusteeRegistry}};


#[derive(Accounts)]
#[instruction(proposal_id:u64,property_system_id:u64)]
pub struct ChangeToNewThreshold<'info>{

    #[account(
        mut,
        // constraint = trustee_registry.trustees.contains(&trustee.key()) @ ErrorCode::NotAuthorized
    )]
    pub trustee:Signer<'info>,

    #[account(
        seeds = [
            TRUSTEE_RECEIPT_SEEDS,
            property_system.key().as_ref(),
            trustee.key().as_ref()
        ],
        bump = trustee_receipt.bump,
    )]
    pub trustee_receipt: Account<'info,TrusteeRecepit>,


    #[account(
        seeds=[
            PROPERTY_SYSTEM_SEEDS,
            &property_system_id.to_le_bytes(),
        ],
        bump = property_system.bump,
    )]
    pub property_system:Account<'info,PropertySystemAccount>,

    // #[account(
    //     seeds=[
    //         TRUSTEEREGISTRYSEEDS,
    //         property_system.key().as_ref()
    //     ],
    //     bump = trustee_registry.bump
    // )]
    // pub trustee_registry :Account<'info,TrusteeRegistry>,

    #[account(
        mut,
        seeds=[
            RT_CHG_PROPOSAL_SEEDS,
            property_system.key().as_ref(),
            &proposal_id.to_le_bytes(),
           
        ],
        bump,
        constraint = proposal.status  == ProposalStatus::Passed @ ErrorCode::ProposalNotPassed
    )]
    pub proposal : Account<'info,RTChgProposal>,
    
    #[account(
        seeds=[
            PROPOSE_THRESHOLD,
            proposal.key().as_ref(),
            new_threshold.signer.as_ref()
        ],
        bump=new_threshold.bump,
    )]
    pub new_threshold : Account<'info,NEWTHRESHOLDPROPOSAL>,

}


pub fn change_to_the_new_threshold(ctx:Context<ChangeToNewThreshold>)->Result<()>{

    let current_time = Clock::get()?.unix_timestamp;

    let  proposal =  &mut ctx.accounts.proposal;

    require!(
        current_time > proposal.voting_for_threshold_deadline &&
        current_time <= proposal.add_new_threshold_deadline,
        ErrorCode::ChangeDeadlineExpire
    );

    let proposal = &mut ctx.accounts.proposal;

    proposal.new_threshold = ctx.accounts.new_threshold.key();


    Ok(())



}
