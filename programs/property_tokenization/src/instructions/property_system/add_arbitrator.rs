use anchor_lang::{ prelude::*};

use crate::{common::{ARBITRAR_RECEIPT_SEEDS, ARBITRAR_REGISTRYSEEDS, PROPERTY_SYSTEM_SEEDS, TRUSTEEREGISTRYSEEDS,}, errors::ErrorCode, state::{ArbitratorRecepit, ArbitratorRegistry, PropertySystemAccount, TrusteeRegistry, }};


#[derive(Accounts)]
#[instruction(system_id:u64)]
pub struct AddArbitrator<'info> {

    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [
            PROPERTY_SYSTEM_SEEDS,
            system_id.to_le_bytes().as_ref()
        ],
        bump = property_system_acc.bump,
        constraint = property_system_acc.creator == authority.key() @ ErrorCode::UnAuthorized,
        constraint = !property_system_acc.ready_for_listing @ ErrorCode::PropertySystemReadyForListing
    )]
    pub property_system_acc: Account<'info, PropertySystemAccount>,

    #[account(
        mut,
        seeds = [
            TRUSTEEREGISTRYSEEDS,
            property_system_acc.key().as_ref()
        ],
        bump = trustee_registry.bump,
        constraint = trustee_registry.property_system_account == property_system_acc.key() @ ErrorCode::InvalidTrusteeRegistry,
        constraint = trustee_registry.current_number_of_trustees == trustee_registry.total_trustees @ ErrorCode::TrusteeLimitNotReached
    )]
    pub trustee_registry: Account<'info, TrusteeRegistry>,

    #[account(
        mut,
        seeds = [
            ARBITRAR_REGISTRYSEEDS,
            property_system_acc.key().as_ref()
        ],
        bump = arbitrator_registry.bump,
        constraint = arbitrator_registry.property_system_account == property_system_acc.key() @ ErrorCode::InvalidTrusteeRegistry
    )]
    pub arbitrator_registry: Account<'info, ArbitratorRegistry>,

    #[account(mut,
    constraint = *new_arbitrator.owner == system_program.key()
    )]
    pub new_arbitrator: Signer<'info>,

    #[account(
        init,
        payer = authority,
        seeds = [
            ARBITRAR_RECEIPT_SEEDS,
            property_system_acc.key().as_ref(),
            new_arbitrator.key().as_ref()
        ],
        bump,
        space = 8 + ArbitratorRecepit::SIZE,   
    )]
    pub new_arbitrator_recepit :Account<'info,ArbitratorRecepit>,

    pub system_program: Program<'info, System>,

}

pub fn add_arbitrator(ctx: Context<AddArbitrator>, _system_id:u64) -> Result<()> {
    let arbitrator_registry = &mut ctx.accounts.arbitrator_registry;

    let property_system_acc = &mut ctx.accounts.property_system_acc;


    require!(arbitrator_registry.total_arbitrators > arbitrator_registry.current_number_of_arbitrators , ErrorCode::AuthorityLimitReached);

    arbitrator_registry.current_number_of_arbitrators += 1;

    let new_arbitrator_recepit = &mut ctx.accounts.new_arbitrator_recepit;

    new_arbitrator_recepit.property_system_account = property_system_acc.key();
    new_arbitrator_recepit.arbitrator = ctx.accounts.new_arbitrator.key();
    new_arbitrator_recepit.bump = ctx.bumps.new_arbitrator_recepit;

    if arbitrator_registry.current_number_of_arbitrators == arbitrator_registry.total_arbitrators     {
        property_system_acc.ready_for_listing = true 
    }

    Ok(())
}