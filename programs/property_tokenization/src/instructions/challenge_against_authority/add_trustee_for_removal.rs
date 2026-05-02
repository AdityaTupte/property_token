use anchor_lang::prelude::*;

use crate::{common::{AuthorityType, CHALLENGEAUTHORITY, OFFENDERRECEIPT, PROPERTY_SYSTEM_SEEDS, ProposalStatus, REMOVEAUTHORITY, REMOVETRUSTEEAUTHORITY, TRUSTEE_RECEIPT_SEEDS}, errors::ErrorCode, state::{ChallengeProposal, ElectAuthority, OffenderReceipt, PropertySystemAccount, Resignation, TrusteeRecepit}};

#[derive(Accounts)]
#[instruction(proposal_id:u64,property_system_id:u64,)]
pub struct AddTrusteeToRemove<'info>{

    #[account(
        mut,
        // constraint = trustee_registry.trustees.contains(&trustee.key()) @ ErrorCode::NotAuthorized
    )]
    pub signer: Signer<'info>,

    pub trustee:SystemAccount<'info>,

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
        seeds = [
            CHALLENGEAUTHORITY,
            property_system.key().as_ref(),
            &proposal_id.to_le_bytes(),
        ],
        bump = challenge_proposal.bump,
        constraint = challenge_proposal.status == ProposalStatus::Executed @ ErrorCode::ProposalNotExecuted
    )]
    pub challenge_proposal: Account<'info, ChallengeProposal>,

    #[account(
        seeds=[
            OFFENDERRECEIPT,
            TRUSTEE_RECEIPT_SEEDS,
            property_system.key().as_ref(),
            challenge_proposal.key().as_ref(),
            trustee.key().as_ref(),
        ],
        bump = trustee_offender_receipt.bump,
        constraint = trustee_offender_receipt.is_finalized @ ErrorCode::NotFinalized
    )]
    pub trustee_offender_receipt :Account<'info,OffenderReceipt>,

    #[account(
        seeds=[
            PROPERTY_SYSTEM_SEEDS,
            &property_system_id.to_le_bytes(),
        ],
        bump = property_system.bump
    )]
    pub property_system : Account<'info,PropertySystemAccount>, 

     #[account(
        init,
        payer = signer,
        seeds=[
            REMOVETRUSTEEAUTHORITY,
            property_system.key().as_ref(),
            trustee.key().as_ref(),  
        ],
        bump,
        space = 8 + Resignation::SIZE
        // constraint = resignation.status ==  ProposalStatus::Pending @ ErrorCode::AlreadyExecuted
    )]
    pub resignation: Account<'info,Resignation>,


    #[account(
        mut,
        seeds=[
            REMOVEAUTHORITY,
            property_system.key().as_ref(),
            challenge_proposal.key().as_ref()
        ],
        bump = proposal.bump ,   
    )]
    pub proposal : Account<'info,ElectAuthority>,

    pub system_program:Program<'info,System>,

} 

pub fn add_trustee_for_removal_proposal(
    ctx:Context<AddTrusteeToRemove>,
    _proposal_id:u64,
    _property_system_id:u64,
    
)->Result<()>{

    let  proposal = &mut ctx.accounts.proposal;
    
    let resignation = &mut ctx.accounts.resignation;

    require!(proposal.status == ProposalStatus::Draft,ErrorCode::NotInDraft);

    require!(proposal.arbitrar_approvals_count == 0, ErrorCode::TotalApprovalCountInvalid);

    resignation.authority = ctx.accounts.trustee.key();

    resignation.property_system = ctx.accounts.property_system.key();

    resignation.authority_type = AuthorityType::TRUSTEE;

    resignation.bump = ctx.bumps.resignation;

    resignation.status = ProposalStatus::Pending;

    resignation.proposal = proposal.key();

    proposal.total_authority_to_resign += 1;



    // let trustee_key = ctx.accounts.trustee.key();

    // require!(
    //     !proposal.authority_to_resign.contains(&trustee_key),
    //     ErrorCode::DuplicateAuthority
    // );

    // require!(
    //     !proposal.authority_to_resign.len() <= 5 as usize ,ErrorCode::AuthorityLimitReached
    // );

    // proposal.authority_to_resign.push(trustee_key);

    

    Ok(())
}
