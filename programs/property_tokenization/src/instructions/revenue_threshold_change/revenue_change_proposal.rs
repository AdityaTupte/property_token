use anchor_lang::prelude::*;

use crate::{common::{PROPERTY_SYSTEM_SEEDS, RT_CHG_PROPOSAL_SEEDS, TRUSTEEREGISTRYSEEDS}, errors::ErrorCode, state::{PropertySystemAccount, RTChgProposal, TrusteeRegistry}};

#[derive(Accounts)]
#[instruction(proposal_id:u64)]
pub struct RTProposal<'info>{

    #[account(
        mut,
        constraint = trustee_registry.trustees.contains(&trustee.key()) @ ErrorCode::NotAuthorized
    )]
    pub trustee:Signer<'info>,

    #[account(
        seeds=[
            PROPERTY_SYSTEM_SEEDS,
            &property_system.property_system_id.to_le_bytes(),
        ],
        bump = property_system.bump,
    )]
    pub property_system : Account<'info,PropertySystemAccount>,

    #[account(
        seeds=[
            TRUSTEEREGISTRYSEEDS,
            property_system.key().as_ref()
        ],
        bump = trustee_registry.bump
    )]
    pub trustee_registry :Account<'info,TrusteeRegistry>,

    #[account(
        init,
        payer = trustee,
        seeds=[
            RT_CHG_PROPOSAL_SEEDS,
            &proposal_id.to_le_bytes(),
            property_system.key().as_ref()
        ],
        bump,
        space = 8 + RTChgProposal::SIZE
    )]
    pub proposal : Account<'info,RTChgProposal>,

    pub system_program : Program<'info,System>,

}

pub fn rt_proposal(
    ctx:Context<RTProposal>,
    proposal_id:u64
)->Result<()>{

    let proposal = &mut ctx.accounts.proposal;

    let property_system = &mut ctx.accounts.property_system;

    proposal.initialize(
        property_system.key(),
        property_system.total_token_supply,
        proposal_id,
        ctx.bumps.proposal, 
       );

    Ok(())

}