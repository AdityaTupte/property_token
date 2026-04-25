use anchor_lang::prelude::*;

use crate::{common::{AuthorityType, PROPERTY_SYSTEM_SEEDS, ProposalStatus, TRUSTEE_RECEIPT_SEEDS, TRUSTEE_RESIGNATION, TRUSTEEREGISTRYSEEDS}, errors::ErrorCode, state::{PropertySystemAccount, Resignation, TrusteeRecepit, TrusteeRegistry}};


#[derive(Accounts)]
#[instruction(property_system_id:u64)]
pub struct TrusteeResign<'info>{

    #[account(
        mut,
    )]
    pub trustee: Signer<'info>,

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
    pub trustee_registry: Account<'info,TrusteeRegistry>,

    #[account(
        init,
        payer = trustee,
        seeds=[
            TRUSTEE_RESIGNATION,
            property_system.key().as_ref(),
            trustee.key().as_ref(),    
        ],
        bump,
        space = 8 + Resignation::SIZE
    )]
    pub resignation: Account<'info,Resignation>,

    pub system_program : Program<'info,System>,

}

pub fn trustee_resign(ctx:Context<TrusteeResign>)->Result<()>{


    let resignation = &mut ctx.accounts.resignation;

    resignation.authority = ctx.accounts.trustee.key();

    resignation.property_system = ctx.accounts.property_system.key();

    resignation.authority_type = AuthorityType::TRUSTEE;

    resignation.bump = ctx.bumps.resignation;

    resignation.status = ProposalStatus::Pending;

    Ok(())
    
}