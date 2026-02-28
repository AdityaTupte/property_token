use anchor_lang::prelude::*;

use crate::constant::{PROPERTY_SEED, ProposalStatus, ProposalType, SELLPROPERTY};
use crate::state::{PropertyAccount, PropertyBuyProposal, PropertySellProposal, ReinvestmentPda, TreasuryPda, TrusteeRegistry};
use crate::{constant::PROPERTY_SYSTEM_SEEDS, state::PropertySystemAccount};
use crate::errors::ErrorCode;


#[derive(Accounts)]
#[instruction(proposal_id : u64)]
pub struct BuyPropertyProposal<'info>{

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
            BUYPROPERTY,
            buyer.key().as_ref(),
            &proposal_id.to_le_bytes()
        ],
        bump,
        space = 8 + PropertyBuyProposal::SIZE
    )]
    pub proposal : Account<'info,PropertyBuyProposal>,

    pub system_program : Program<'info,System>,

}


pub fn createbuyproposal(ctx:Context<BuyPropertyProposal>,proposal_id : u64) ->Result<()>{

    let buy_proposal = &mut ctx.accounts.proposal;
    
    let buyer = & ctx.accounts.buyer;

    let buyer_wallet = & ctx.accounts.buyer_reinvestment_pda;

    let seller_proposal = & ctx.accounts.seller_proposal;

    let property = & ctx.accounts.property;

    let current_time = Clock::get()?.unix_timestamp ;

    require!(current_time < seller_proposal.transfer_deadline , ErrorCode::TransferDeadLineReached);

    buy_proposal.initialize(
        proposal_id,
        buyer.key(),
        buyer_wallet.key(),
        property.key(),
        seller_proposal.key(),
        seller_proposal.sale_price,
        ctx.bumps.proposal,
        buyer.total_token_supply,
    );


    Ok(())
}