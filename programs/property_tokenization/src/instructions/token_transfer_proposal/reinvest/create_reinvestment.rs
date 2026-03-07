use anchor_lang::prelude::*;

use crate::{common::{PROPERTY_SYSTEM_SEEDS, USEREINVESTMENTOKEN}, errors::ErrorCode, state::{PropertySystemAccount, TrusteeRegistry, UseReinvestmentProposal}};

#[derive(Accounts)]
#[instruction(proposal_id:u64)]
pub struct UseReinvestmentTokensProposal<'info>{

    #[account(
        mut,
        constraint = trustee_registry.trustees.contains(&trustee.key()) @ ErrorCode::NotAuthorized
    )]
    pub trustee: Signer<'info>,


    #[account(
        seeds=[
            PROPERTY_SYSTEM_SEEDS,
            &property_system.property_system_id.to_le_bytes(),
        ],
        bump = property_system.bump
    )]
    pub property_system: Account<'info,PropertySystemAccount>,

    #[account(
        constraint =  property_system.trustee_registry == trustee_registry.key() @ ErrorCode::InvalidTrusteeRegsitry
    )]
    pub trustee_registry : Account<'info,TrusteeRegistry>,


    #[account(
        init,
        payer = trustee,
        seeds=[
                USEREINVESTMENTOKEN,
                property_system.key().as_ref(),
                &proposal_id.to_le_bytes(),
        ],
        bump,
        space = 8 + UseReinvestmentProposal::SIZE
    )]
    pub proposal : Account<'info,UseReinvestmentProposal>,

    pub receipent_wallet : SystemAccount<'info>,

    pub system_program:Program<'info,System>,

}



pub fn create_use_reinvest_proposal(
    ctx:Context<UseReinvestmentTokensProposal>,
    proposal_id :u64,
    amount_required:u64,
    reason_hash:[u8;32],
)->Result<()>{

    let property_system = &mut ctx.accounts.property_system;

    let receipent_wallet = &mut ctx.accounts.receipent_wallet;

    let proposal = &mut ctx.accounts.proposal;

    proposal.initialize(proposal_id, property_system.key(), amount_required, reason_hash, *receipent_wallet.key,ctx.bumps.proposal,property_system.total_token_supply);
    
    Ok(())

}



