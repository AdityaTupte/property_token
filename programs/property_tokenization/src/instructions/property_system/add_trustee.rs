use anchor_lang::{ prelude::*};

use crate::{common::{PROPERTY_SYSTEM_SEEDS, TRUSTEE_RECEIPT_SEEDS, TRUSTEEREGISTRYSEEDS}, errors::ErrorCode, state::{PropertySystemAccount, TrusteeRecepit, TrusteeRegistry}};


#[derive(Accounts)]
#[instruction(system_id:u64)]
pub struct AddTrustee<'info> {

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
        constraint = trustee_registry.property_system_account == property_system_acc.key() @ ErrorCode::InvalidTrusteeRegistry
    )]
    pub trustee_registry: Account<'info, TrusteeRegistry>,

    #[account(
    mut,
    constraint = *new_trustee.owner == system_program.key()
    )]
    pub new_trustee: Signer<'info>,
    
    #[account(
        init,
        payer = authority,
        seeds = [
            TRUSTEE_RECEIPT_SEEDS,
            property_system_acc.key().as_ref(),
            new_trustee.key().as_ref()
        ],
        bump,
        space = 8 + TrusteeRecepit::SIZE,   
    )]
    pub new_trustee_recepit :Account<'info,TrusteeRecepit>,


    pub system_program: Program<'info, System>,

}

pub fn add_trustee(ctx: Context<AddTrustee>, _system_id:u64) -> Result<()> {

    let trustee_registry = &mut ctx.accounts.trustee_registry;

    require!(trustee_registry.total_trustees > trustee_registry.current_number_of_trustees as u8, ErrorCode::AuthorityLimitReached);

    trustee_registry.current_number_of_trustees += 1;

    let new_trustee_recepit = &mut ctx.accounts.new_trustee_recepit;

    new_trustee_recepit.property_system_account = ctx.accounts.property_system_acc.key();
    new_trustee_recepit.trustee = ctx.accounts.new_trustee.key();
    new_trustee_recepit.bump = ctx.bumps.new_trustee_recepit;

    Ok(())
}