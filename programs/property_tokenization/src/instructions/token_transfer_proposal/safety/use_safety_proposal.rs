use anchor_lang::prelude::*;

use crate::{common::{PROPERTY_SYSTEM_SEEDS, SAFETYPROPOSAL, TRUSTEEREGISTRYSEEDS, TRUSTEE_RECEIPT_SEEDS}, state::{PropertySystemAccount, SafetyProposal, TrusteeRecepit, TrusteeRegistry}};

#[derive(Accounts)]
#[instruction(proposal_id:u64,property_system_id:u64,)]
pub struct UseSafetyTokensProposal<'info>{

    #[account(
        mut,
        // constraint = trustee_registry.trustees.contains(&trustee.key()) @ ErrorCode::NotAuthorized
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
    pub property_system: Account<'info,PropertySystemAccount>,

    #[account(
        seeds=[
            TRUSTEEREGISTRYSEEDS,
            property_system.key().as_ref()
        ],
        bump,
        // constraint =  property_system.trustee_registry == trustee_registry.key() @ ErrorCode::InvalidTrusteeRegistry
    )]
    pub trustee_registry : Account<'info,TrusteeRegistry>,


    #[account(
        init,
        payer = trustee,
        seeds=[
                SAFETYPROPOSAL,
                property_system.key().as_ref(),
                &proposal_id.to_le_bytes(),
        ],
        bump,
        space = 8 + SafetyProposal::SIZE
    )]
    pub proposal : Account<'info,SafetyProposal>,

    pub receipent_wallet : SystemAccount<'info>,

    pub system_program:Program<'info,System>,

}



pub fn create_use_safety_proposal(
    ctx:Context<UseSafetyTokensProposal>,
     proposal_id :u64,
    _property_system_id:u64,
    amount_required:u64,
    reason_hash:[u8;32],
)->Result<()>{

    let property_system = &mut ctx.accounts.property_system;

    let receipent_wallet = &mut ctx.accounts.receipent_wallet;

    let proposal = &mut ctx.accounts.proposal;


    proposal.initialize(proposal_id, property_system.key(), amount_required, reason_hash, *receipent_wallet.key,ctx.bumps.proposal,property_system.total_token_supply);
    
    Ok(())

}

