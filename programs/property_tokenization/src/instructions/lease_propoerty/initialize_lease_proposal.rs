use anchor_lang::prelude::*;

use crate::{common::{ LEASE_PROPERTY_PROPOSAL, PROPERTY_PAGE_SEEDS, PROPERTY_SEED, PROPERTY_SYSTEM_SEEDS, ProposalStatus, TRUSTEEREGISTRYSEEDS}, errors::ErrorCode, state::{ LeaseProposal, PropertyAccount, PropertyPage, PropertySystemAccount, TrusteeRegistry}};


#[derive(Accounts)]
#[instruction(lease_id:u64)]
pub struct InitializeLeaseProposal<'info>{

    #[account(
        mut,
        constraint = trustee_registry.trustees.contains(&signer.key()) @ ErrorCode::NotAuthorized
    )]
    pub signer : Signer<'info>,

    pub lessee : SystemAccount<'info>,

    pub neutral : SystemAccount<'info>,

    #[account(
        seeds=[
            PROPERTY_SYSTEM_SEEDS,
            &property_system.property_system_id.to_le_bytes(),
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
                    &property.property_id.to_le_bytes(),
                    property.state_pubkey.as_ref(),
            ],
            bump = property.bump,
            constraint = property.property_system == property_system.key() @ ErrorCode::PropertySystemInvalid,
            constraint = !property.is_leased @ ErrorCode::LeaseActivated
    )]
    pub property : Account<'info,PropertyAccount>,

    #[account(
        seeds=[
                PROPERTY_PAGE_SEEDS,
                &property_page.page.to_le_bytes(),
                property_system.key().as_ref()
        ],
        bump= property_page.bump,
        constraint = property_page.land.contains(&property.key()) @ ErrorCode::LandAccountNotFound
    )]
    pub property_page : Account<'info,PropertyPage>,

    #[account(
        init,
        payer = signer,
        seeds=[
            LEASE_PROPERTY_PROPOSAL,
            &lease_id.to_le_bytes(),
            property.key().as_ref(),
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
    rent : u64,
    security_deposit:u64,
    agreement_hash :[u8;32],
    end_time_in_days : u32,
)->Result<()>{


    let lease_proposal = &mut ctx.accounts.lease_proposal;

    lease_proposal.initailized_at =Clock::get()?.unix_timestamp;
    
    lease_proposal.property_system = ctx.accounts.property_system.key();

    lease_proposal.lease_id = lease_id;

    lease_proposal.rent_amount = rent;

    lease_proposal.security_deposit = security_deposit;

    lease_proposal.agreemenbt_hash = agreement_hash;

    lease_proposal.neutral = ctx.accounts.neutral.key();

    lease_proposal.lease_end_time = (end_time_in_days as i64)
                                .checked_mul(24*60*60)
                                .ok_or(ErrorCode::MathOverflow)?;
                   
    lease_proposal.status = ProposalStatus::Draft;

    Ok(())



}