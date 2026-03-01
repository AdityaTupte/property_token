use anchor_lang::prelude::*;

use crate::{constant::*, state::{PropertyBuyProposal, PropertySystemAccount, TrusteeRegistry}};


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

    #[account()]
    pub property_system : Account<'info,PropertySystemAccount>,

    #[account(
        mut,
        seeds=[
            BUYPROPERTY,
            property_system.key().as_ref(),
            &proposal.proposal_id.to_le_bytes(),
        ],
        bump = proposal.bump,
        constraint = proposal.buyer  == property_system.key(),
        close = trustee
    )]
    pub proposal : Account<'info,PropertyBuyProposal>,

}  


pub fn delete_buy_proposal(ctx:Context<DeleteFailProposal>) ->Result<()>{

    let proposal = &mut *ctx.accounts.proposal;

    delete_proposal(proposal)?;

    Ok(())

}