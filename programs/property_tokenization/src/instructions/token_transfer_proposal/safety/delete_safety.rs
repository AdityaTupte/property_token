use anchor_lang::prelude::*;

use crate::{common::{PROPERTY_SYSTEM_SEEDS, SAFETYPROPOSAL, TRUSTEE_RECEIPT_SEEDS}, functions::delete_proposal, state::{PropertySystemAccount, TokenTransferProposal, TrusteeRecepit, TrusteeRegistry}};


#[derive(Accounts)]
#[instruction(proposal_id:u64,property_system_id:u64)]
pub struct DeleteSafetyFailProposal<'info>{

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
        mut,
        // constraint = trustee_registry.trustees.contains(&trustee.key())
    )]
    pub trustee : Signer<'info>,

    #[account(
        seeds=[
            PROPERTY_SYSTEM_SEEDS,
            &property_system_id.to_le_bytes(),
        ],
        bump = property_system.bump
    )]
    pub property_system : Account<'info,PropertySystemAccount>,

    #[account(
        mut,
        seeds=[
            SAFETYPROPOSAL,
            property_system.key().as_ref(),
            &proposal_id.to_le_bytes(),
        ],
        bump = proposal.bump,
        constraint = proposal.property_system  == property_system.key(),
        close = trustee
    )]
    pub proposal : Account<'info,TokenTransferProposal>,

}  


pub fn delete_buy_safety_proposal(
    ctx:Context<DeleteSafetyFailProposal>,
    _proposal_id:u64,_property_system_id:u64
) ->Result<()>{

    let proposal = &mut *ctx.accounts.proposal;

    delete_proposal(proposal)?;

    Ok(())

}