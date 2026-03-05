use anchor_lang::prelude::*;

use crate::{constant::*, functions::delete_proposal, state::{PropertySystemAccount, TrusteeRegistry, UseReinvestmentProposal}};


#[derive(Accounts)]
pub struct DeleteFailProposal<'info>{

    #[account(
        constraint = property_system.trustee_registry == trustee_registry.key() 
    )]
    pub trustee_registry : Account<'info,TrusteeRegistry>,

    #[account(
        mut,
        constraint = trustee_registry.trustees.contains(&trustee.key())
    )]
    pub trustee : Signer<'info>,

    #[account(
        seeds=[
            PROPERTY_SYSTEM_SEEDS,
            &property_system.property_system_id.to_le_bytes(),
        ],
        bump = property_system.bump
    )]
    pub property_system : Account<'info,PropertySystemAccount>,

    #[account(
        mut,
        seeds=[
            USEREINVESTMENTOKEN,
            property_system.key().as_ref(),
            &proposal.proposal_id.to_le_bytes(),
        ],
        bump = proposal.bump,
        constraint = proposal.property_system  == property_system.key(),
        close = trustee
    )]
    pub proposal : Account<'info,UseReinvestmentProposal>,

}  


pub fn delete_buy_proposal(ctx:Context<DeleteFailProposal>) ->Result<()>{

    let proposal = &mut *ctx.accounts.proposal;

    delete_proposal(proposal)?;

    Ok(())

}