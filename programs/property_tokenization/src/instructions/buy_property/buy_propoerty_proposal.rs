use anchor_lang::prelude::*;

use crate::constant::{PROPERTY_SEED, ProposalStatus, ProposalType, SELLPROPERTY};
use crate::state::{PropertyAccount, PropertySellProposal, ReinvestmentPda, TreasuryPda, TrusteeRegistry};
use crate::{constant::PROPERTY_SYSTEM_SEEDS, state::PropertySystemAccount};
use crate::errors::ErrorCode;


#[derive(Accounts)]

pub struct PropertyBuyProposal<'info>{

    #[account(
        mut,
        constraint = trustee_registry.trustees.contains(&trustee.key()) @ ErrorCode::NotAuthorized
    )]
    pub trustee: Signer<'info>,


    #[account(
        seeds =[
                PROPERTY_SYSTEM_SEEDS,
                &buyer.property_system_id.to_le_bytes()
        ],
        bump = buyer.bump,
    )]
    pub buyer:Account<'info,PropertySystemAccount>,

    #[account(
        constraint = buyer.trustee_registry == trustee_registry.key() @ ErrorCode::InvalidTrusteeRegsitry
    )]
    pub trustee_registry: Account<'info,TrusteeRegistry>,

    #[account(
        constraint = buyer.treasury == buyer_treasury.key() @ ErrorCode::InvalidTreasury
    )]
    pub buyer_treasury: Account<'info,TreasuryPda>,

    #[account(
        constraint = buyer_treasury.reinvenstement_acc == buyer_reinvestment_pda.key() @ ErrorCode::InvalidReinvestAccount
    )]
    pub buyer_reinvestment_pda:Account<'info,ReinvestmentPda>,
    
    #[account(
        seeds=[
            SELLPROPERTY,
            seller_proposal.property_system_account.as_ref(),
            &seller_proposal.proposal_id.to_le_bytes(),  
        ],
        bump = seller_proposal.bump,
        constraint = seller_proposal.property_account == property.key() @ ErrorCode::InvalidProperty,
        constraint = seller_proposal.proposal_type == ProposalType::SELLPROPERTY,
        constraint = seller_proposal.status == ProposalStatus::Passed @ ErrorCode::PropertyNotPass,
    )]
    pub seller_proposal : Account<'info,PropertySellProposal>,

    #[account(
        seeds=[
                PROPERTY_SEED,
                &property.property_id.to_le_bytes(),
                property.state_pubkey.as_ref(),
                property.country_pubkey.as_ref()
        ],
        bump = property.bump,
        constraint = seller_proposal.property_system_account == property.property_system @ ErrorCode::InvalidProperty
    )]
    pub property : Account<'info,PropertyAccount>,


    #[account(
        init,
        payer = trustee,
        seeds=[
            BUY
        ],
        bump,
        space = 8 + 
    )]
    pub 


}


pub fn buyproposal(ctx:Context<PropertyBuyProposal>) ->Result<()>{


    Ok(())
}