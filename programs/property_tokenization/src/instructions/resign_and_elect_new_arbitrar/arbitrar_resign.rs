use anchor_lang::prelude::*;

use crate::{common::{ARBITRAR_REGISTRYSEEDS, ARBITRAR_RESIGNATION, AuthorityType, PROPERTY_SYSTEM_SEEDS, ProposalStatus,}, errors::ErrorCode, state::{ArbitratorRegistry, PropertySystemAccount, Resignation,}};


#[derive(Accounts)]
pub struct ArbitrarResign<'info>{

    #[account(
        mut,
        constraint = arbitrar_registry.arbitrator.contains(&arbitrar.key()) @ ErrorCode::NotAuthorized
    )]
    pub arbitrar: Signer<'info>,

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
            ARBITRAR_REGISTRYSEEDS,
            property_system.key().as_ref()
        ],
        bump = arbitrar_registry.bump
    )]
    pub arbitrar_registry: Account<'info,ArbitratorRegistry>,

    #[account(
        init,
        payer = arbitrar,
        seeds=[
            ARBITRAR_RESIGNATION,
            arbitrar.key().as_ref(),
            property_system.key().as_ref(),
        ],
        bump,
        space = 8 + Resignation::SIZE
    )]
    pub resignation: Account<'info,Resignation>,

    pub system_program : Program<'info,System>,

}

pub fn arbitrar_resign(ctx:Context<ArbitrarResign>)->Result<()>{


    let resignation = &mut ctx.accounts.resignation;

    resignation.authority = ctx.accounts.arbitrar.key();

    resignation.property_system = ctx.accounts.property_system.key();

    resignation.authority_type = AuthorityType::ARBITRATOR;

    resignation.bump = ctx.bumps.resignation;

    resignation.status = ProposalStatus::Pending;

    Ok(())
    
}