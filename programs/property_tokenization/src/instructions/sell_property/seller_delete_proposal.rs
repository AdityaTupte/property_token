use anchor_lang::prelude::*;

use crate::{ common::{PROPERTY_SYSTEM_SEEDS, SELLPROPERTY, TRUSTEEREGISTRYSEEDS}, functions::delete_proposal, state::{PropertySellProposal, PropertySystemAccount, TrusteeRegistry}};


#[derive(Accounts)]
pub struct DeleteFailProposal<'info>{

    #[account(
        seeds = [
            TRUSTEEREGISTRYSEEDS,
            property_system.key().as_ref()
            ],
        bump = trustee_registry.bump,
    )]
    pub trustee_registry : Account<'info,TrusteeRegistry>,

    #[account(
        mut,
        constraint = trustee_registry.trustees.contains(&trustee.key())
    )]
    pub trustee : Signer<'info>,

    #[account(
        seeds = [ 
            PROPERTY_SYSTEM_SEEDS,
            &property_system.property_system_id.to_le_bytes(),
        ],
        bump=property_system.bump,
    )]
    pub property_system : Account<'info,PropertySystemAccount>,

    #[account(
        mut,
        seeds=[
            SELLPROPERTY,
            property_system.key().as_ref(),
            &proposal.proposal_id.to_le_bytes(),
        ],
        bump = proposal.bump,
        close = trustee
    )]
    pub proposal : Account<'info,PropertySellProposal>,

}

pub fn delete_fail_proposal(ctx:Context<DeleteFailProposal>)->Result<()>{

    let proposal = &mut *ctx.accounts.proposal;

    delete_proposal(proposal)?;


    Ok(())

}


