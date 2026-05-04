use anchor_lang::prelude::*;

use crate::{common::{ AuthorityType, CHALLENGEAUTHORITY, OFFENDERRECEIPT, PROPERTY_SYSTEM_SEEDS, ProposalStatus, TRUSTEE_RECEIPT_SEEDS}, errors::ErrorCode, state::{ ChallengeProposal, OffenderReceipt, PropertySystemAccount, TrusteeRecepit}};



#[derive(Accounts)]
#[instruction(proposal_id : u64,property_system_id : u64)]
pub struct AddTrusteeOffender<'info>{

    #[account(
        mut,
        constraint = signer.key() == proposal.creator @ ErrorCode::UnAuthorized
    )]
    pub signer :Signer<'info>,


    pub trustee_offender : SystemAccount<'info>,

    #[account(
        seeds=[
            TRUSTEE_RECEIPT_SEEDS,
            property_system.key().as_ref(),
            trustee_offender.key().as_ref()
        ],
        bump = trustee_receipt.bump
    )]
    pub trustee_receipt :Account<'info,TrusteeRecepit>,

    
    #[account(
        init,
        payer=signer,
        seeds=[
            OFFENDERRECEIPT,
            TRUSTEE_RECEIPT_SEEDS,
            property_system.key().as_ref(),
            proposal.key().as_ref(),
            trustee_offender.key().as_ref()
        ],
        bump,
        space = 8 + OffenderReceipt::SIZE
    )]
    pub offender_receipt :Account<'info,OffenderReceipt>,

    // pub arbitrar_offender : SystemAccount<'info>,

  


    #[account(
        mut,
        seeds=[
            CHALLENGEAUTHORITY,
            property_system.key().as_ref(),
            &proposal_id.to_le_bytes()
        ],
        bump= proposal.bump,
        constraint = proposal.status == ProposalStatus::Draft @ ErrorCode::NotInDraft
    )]
    pub proposal : Account<'info,ChallengeProposal>,

    #[account(
        seeds=[
            PROPERTY_SYSTEM_SEEDS,
            &property_system_id.to_le_bytes()
        ],
        bump = property_system.bump,
    )]
    pub property_system : Account<'info,PropertySystemAccount>,


    pub system_program : Program<'info,System>,

}


pub fn add_trustee_offender(
    ctx:Context<AddTrusteeOffender>,
    _proposal_id : u64,_property_system_id : u64
)->Result<()>{

    let proposal = &mut ctx.accounts.proposal;

    let offender_receipt = &mut ctx.accounts.offender_receipt;

    proposal.trustee_offender_total_number += 1;

    offender_receipt.offender_key = ctx.accounts.trustee_offender.key();

    offender_receipt.proposal_key = proposal.key();

    offender_receipt.property_system_key = ctx.accounts.property_system.key();

    offender_receipt.authority_type = AuthorityType::TRUSTEE;

    offender_receipt.bump = ctx.bumps.offender_receipt;

    Ok(())

}