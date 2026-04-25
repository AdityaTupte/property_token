use anchor_lang::prelude::*;

use crate::{common::{BUYPROPERTY, PROPERTY_SYSTEM_SEEDS, TRUSTEE_RECEIPT_SEEDS}, state::{PropertyBuyProposal, PropertySystemAccount, TrusteeRecepit}};

use crate::functions::*;

#[derive(Accounts)]
#[instruction(proposal_id:u64,property_system_id:u64)]
pub struct DeleteFailBuyProposal<'info>{

    #[account(
        mut,
        // constraint = trustee_registry.trustees.contains(&trustee.key())
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


    #[account(
        seeds = [ 
            PROPERTY_SYSTEM_SEEDS,
            &property_system_id.to_le_bytes(),
        ],
        bump=property_system.bump,
    )]
    pub property_system : Account<'info,PropertySystemAccount>,

    #[account(
        mut,
        seeds=[
            BUYPROPERTY,
            property_system.key().as_ref(),
            &proposal_id.to_le_bytes(),
        ],
        bump = proposal.bump,
        close = trustee
    )]
    pub proposal : Account<'info,PropertyBuyProposal>,

}  


pub fn delete_buy_proposal(ctx:Context<DeleteFailBuyProposal>,_proposal_id:u64,_property_system_id:u64) ->Result<()>{

    let proposal = &mut *ctx.accounts.proposal;

    delete_proposal(proposal)?;

    Ok(())

}