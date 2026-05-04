    use anchor_lang::prelude::*;

use crate::{common::{ARBITRAR_RECEIPT_SEEDS, AuthorityType, CHALLENGEAUTHORITY, OFFENDERRECEIPT, PROPERTY_SYSTEM_SEEDS, ProposalStatus, REMOVEARBITRARAUTHORITY, REMOVEARBITRARAUTHORITYPROPOSAL,}, errors::ErrorCode, state::{ArbitratorRecepit, ChallengeProposal, ElectAuthority, OffenderReceipt, PropertySystemAccount, Resignation}};

#[derive(Accounts)]
#[instruction(proposal_id:u64,property_system_id:u64,)]
pub struct AddArbitrarToRemove<'info>{

    #[account(
        mut,
        // constraint = trustee_registry.trustees.contains(&trustee.key()) @ ErrorCode::NotAuthorized
    )]
    pub signer: Signer<'info>,

    pub arbitrar:SystemAccount<'info>,

    #[account(
        seeds = [
            ARBITRAR_RECEIPT_SEEDS,
            property_system.key().as_ref(),
            arbitrar.key().as_ref()
        ],
        bump = arbitrar_receipt.bump,
    )]
    pub arbitrar_receipt: Account<'info,ArbitratorRecepit>,

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
            ARBITRAR_RECEIPT_SEEDS,
            property_system.key().as_ref(),
            challenge_proposal.key().as_ref(),
            arbitrar.key().as_ref(),
        ],
        bump = arbitrar_offender_receipt.bump,
        constraint = arbitrar_offender_receipt.is_finalized @ ErrorCode::NotFinalized
    )]
    pub arbitrar_offender_receipt :Account<'info,OffenderReceipt>,

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
            REMOVEARBITRARAUTHORITY,
            property_system.key().as_ref(),
            arbitrar.key().as_ref(),  
        ],
        bump,
        space = 8 + Resignation::SIZE
        // constraint = resignation.status ==  ProposalStatus::Pending @ ErrorCode::AlreadyExecuted
    )]
    pub resignation: Account<'info,Resignation>,


    #[account(
        mut,
        seeds=[
            REMOVEARBITRARAUTHORITYPROPOSAL,
            property_system.key().as_ref(),
            challenge_proposal.key().as_ref()
        ],
        bump = proposal.bump ,   
    )]
    pub proposal : Account<'info,ElectAuthority>,

    pub system_program:Program<'info,System>,

} 

pub fn add_arbitrar_for_removal_proposal(
    ctx:Context<AddArbitrarToRemove>,
    _proposal_id:u64,
    _property_system_id:u64,
    
)->Result<()>{

    let  proposal = &mut ctx.accounts.proposal;
    
    let resignation = &mut ctx.accounts.resignation;

    require!(proposal.status == ProposalStatus::Draft,ErrorCode::NotInDraft);

    require!(proposal.arbitrar_approvals_count == 0, ErrorCode::TotalApprovalCountInvalid);

    resignation.authority = ctx.accounts.arbitrar.key();

    resignation.property_system = ctx.accounts.property_system.key();

    resignation.authority_type = AuthorityType::ARBITRATOR;

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
