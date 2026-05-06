use anchor_lang::prelude::*;

use crate::{common::{ LEASE_PROPERTY_PROPOSAL, PROPERTY_SEED, PROPERTY_SYSTEM_SEEDS, ProposalStatus, TRUSTEE_RECEIPT_SEEDS, TRUSTEEREGISTRYSEEDS}, errors::ErrorCode, state::{ LeaseProposal, PropertyAccount,  PropertySystemAccount, TrusteeRecepit, TrusteeRegistry}};


#[derive(Accounts)]
#[instruction(lease_id:u64,property_id:u64,state_pubkey:Pubkey,property_system_id:u64)]
pub struct InitializeLeaseProposal<'info>{

    #[account(
        mut,
        // constraint = trustee_registry.trustees.contains(&signer.key()) @ ErrorCode::NotAuthorized
    )]
    pub trustee : Signer<'info>,

     #[account(
        seeds = [
            TRUSTEE_RECEIPT_SEEDS,
            property_system.key().as_ref(),
            trustee.key().as_ref()
        ],
        bump = trustee_receipt.bump,
    )]
    pub trustee_receipt: Account<'info,TrusteeRecepit>,

    pub lessee : SystemAccount<'info>,

    pub neutral : SystemAccount<'info>,

    #[account(
        seeds=[
            PROPERTY_SYSTEM_SEEDS,
            &property_system_id.to_le_bytes(),
        ],
        bump = property_system.bump
    )]
    pub property_system : Account<'info,PropertySystemAccount>,

    #[account(
        seeds=[
            TRUSTEEREGISTRYSEEDS,
            property_system.key().as_ref()
        ],
        bump = trustee_registry.bump
    )]
    pub trustee_registry : Account<'info,TrusteeRegistry>,

    #[account(
        seeds = [
                    PROPERTY_SEED,
                    &property_id.to_le_bytes(),
                    state_pubkey.as_ref(),
            ],
            bump = property.bump,
            constraint = property.property_system == property_system.key() @ ErrorCode::PropertySystemInvalid,
            constraint = !property.is_leased @ ErrorCode::LeaseActivated
    )]
    pub property : Account<'info,PropertyAccount>,

    // #[account(
    //     seeds=[
    //             PROPERTY_PAGE_SEEDS,
    //             &property_page.page.to_le_bytes(),
    //             property_system.key().as_ref()
    //     ],
    //     bump= property_page.bump,
    //     constraint = property_page.land.contains(&property.key()) @ ErrorCode::LandAccountNotFound
    // )]
    // pub property_page : Account<'info,PropertyPage>,

    #[account(
        init,
        payer = trustee,
        seeds=[
            LEASE_PROPERTY_PROPOSAL,
            property_system.key().as_ref(),
            property.key().as_ref(),
            &lease_id.to_le_bytes(),
        ],
        bump ,
        space = 8 + LeaseProposal::SIZE
    )]
    pub lease_proposal : Account<'info,LeaseProposal>,

    pub system_program : Program<'info,System>,

}


pub fn initialize_lease_proposal(
    ctx:Context<InitializeLeaseProposal>,
    lease_id :u64,
    _property_id:u64,
    _state_pubkey:Pubkey,
    _property_system_id:u64,
    rent : u64,
    security_deposit:u64,
    agreement_hash :[u8;32],
    end_time_in_days : u32,
    late_payment_fee_per_day : u64,
    periodic_pay : i64,
)->Result<()>{


    let lease_proposal = &mut ctx.accounts.lease_proposal;

    let now = Clock::get()?.unix_timestamp;

    lease_proposal.initailized_at = now;

    lease_proposal.property = ctx.accounts.property.key();

    lease_proposal.lessee = ctx.accounts.lessee.key();
    
    lease_proposal.property_system = ctx.accounts.property_system.key();

    lease_proposal.lease_id = lease_id;

    lease_proposal.rent_amount = rent;

    lease_proposal.security_deposit = security_deposit;

    lease_proposal.agreemenbt_hash = agreement_hash;

    lease_proposal.neutral = ctx.accounts.neutral.key();

    let lease_duration_in_sec = (end_time_in_days as i64)
                                .checked_mul(24*60*60)
                                .ok_or(ErrorCode::MathOverflow)?;

    lease_proposal.lease_end_time = now
                                .checked_add(lease_duration_in_sec)
                                .ok_or(ErrorCode::MathOverflow)?;
                   
    lease_proposal.periodic_pay = periodic_pay;

    lease_proposal.late_payment_fee_per_day = late_payment_fee_per_day;

    lease_proposal.status = ProposalStatus::Draft;

    lease_proposal.bump = ctx.bumps.lease_proposal;

    Ok(())



}
