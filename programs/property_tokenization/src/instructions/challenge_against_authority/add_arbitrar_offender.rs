use anchor_lang::prelude::*;

use crate::{common::{ ARBITRAR_RECEIPT_SEEDS, AuthorityType, CHALLENGEAUTHORITY, OFFENDERRECEIPT, PROPERTY_SYSTEM_SEEDS, ProposalStatus, TRUSTEE_RECEIPT_SEEDS}, errors::ErrorCode, state::{ArbitratorRecepit, ChallengeProposal, OffenderReceipt, PropertySystemAccount, TrusteeRecepit}};



#[derive(Accounts)]
#[instruction(proposal_id : u64,property_system_id : u64)]
pub struct AddArbitrarOffender<'info>{

    #[account(
        mut,
        constraint = signer.key() == proposal.creator @ ErrorCode::UnAuthorized
    )]
    pub signer :Signer<'info>,


    pub arbitrar_offender : SystemAccount<'info>,

      #[account(
        seeds=[
            ARBITRAR_RECEIPT_SEEDS,
            property_system.key().as_ref(),
            arbitrar_offender.key().as_ref()
        ],
        bump = arbitrar_receipt.bump
    )]
    pub arbitrar_receipt :Account<'info,ArbitratorRecepit>,

    
    #[account(
        init,
        payer=signer,
        seeds=[
            OFFENDERRECEIPT,
            ARBITRAR_RECEIPT_SEEDS,
            property_system.key().as_ref(),
            proposal.key().as_ref(),
            arbitrar_offender.key().as_ref(),
        ],
        bump,
        space = 8 + OffenderReceipt::SIZE
    )]
    pub offender_receipt :Account<'info,OffenderReceipt>,

    // pub arbitrar_offender : SystemAccount<'info>,

    // #[account(
    //     seeds=[
    //         ARBITRAR_RECEIPT_SEEDS,
    //         property_system.key().as_ref(),
    //         arbitrar_offender.key().as_ref()
    //     ],
    //     bump = arbitrar_receipt.bump
    // )]
    // pub arbitrar_receipt :Account<'info,ArbitratorRecepit>,


    #[account(
        mut,
        seeds=[
            CHALLENGEAUTHORITY,
            property_system.key().as_ref(),
            &proposal_id.to_le_bytes(),
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


pub fn add_arbitrar_offender(
    ctx:Context<AddArbitrarOffender>,
    _proposal_id : u64,_property_system_id : u64
)->Result<()>{

    let proposal = &mut ctx.accounts.proposal;

    let offender_receipt = &mut ctx.accounts.offender_receipt;

    proposal.arbitrar_offender_total_number += 1;

    offender_receipt.offender_key = ctx.accounts.arbitrar_offender.key();

    offender_receipt.proposal_key = proposal.key();

    offender_receipt.property_system_key = ctx.accounts.property_system.key();

    offender_receipt.authority_type = AuthorityType::ARBITRATOR;

    offender_receipt.bump = ctx.bumps.offender_receipt;

    Ok(())

}