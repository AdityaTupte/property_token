use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token_interface::{TokenAccount}};

use crate::{constant::{HARDCODED_PUBKEY, PROPERTY_PAGE_SEEDS, PROPERTY_SEED, PROPERTY_SYSTEM_SEEDS, SELLPROPERTY}, state::{PropertyAccount, PropertyBuyProposal, PropertyPage, PropertySellProposal, PropertySystemAccount, ReinvestmentPda, TreasuryPda, TrusteeRegistry}};

use crate::errors::ErrorCode;
#[derive(Accounts)]

pub struct ExecuteProposal<'info>{

     #[account(
        constraint = buyer.trustee_registry == trustee_registry.key() 
    )]
    pub trustee_registry : Account<'info,TrusteeRegistry>,

    #[account(
        mut,
        constraint = trustee_registry.trustees.contains(&trustee.key())
    )]
    pub trustee : Signer<'info>,

    #[account(
        mut,
        seeds =[
                PROPERTY_SYSTEM_SEEDS,
                &buyer.property_system_id.to_le_bytes()
        ],
        bump = buyer.bump,
    )]
    pub buyer : Account<'info,PropertySystemAccount>,

    #[account(
        constraint = buyer.treasury == buyer_treasury.key() @ ErrorCode::InvalidTreasury
    )]
    pub buyer_treasury: Account<'info,TreasuryPda>,

    #[account(
        mut,
        constraint = buyer_wallet.key() ==  buyer_treasury.reinvenstement_acc @ ErrorCode::InvalidReinvestAccount 
    )]
    pub buyer_wallet : Account<'info,ReinvestmentPda>,

    #[account(
        mut,
        seeds=[
            BUYPROPERTY,
            buyer.key().as_ref(),
            &proposal.proposal_id.to_le_bytes(),
        ],
        bump = proposal.bump,
        constraint = proposal.buyer  == buyer.key(),
    )]
    pub proposal : Account<'info,PropertyBuyProposal>,

    #[account(
        associated_token::mint = HARDCODED_PUBKEY,
        associated_token::authority = buyer_wallet ,
    )]
    pub buyer_ata : InterfaceAccount<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    
    pub system_program: Program<'info, System>,
    ///
    /// 
    /// 
    /// 
    #[account(
         mut,
        seeds=[
            SELLPROPERTY,
            seller.key().as_ref(),
            &sell_proposal.proposal_id.to_le_bytes(),
        ],
        bump = sell_proposal.bump,
        constraint = sell_proposal.property_system_account  == buyer.key(),
    )]
    pub sell_proposal: Account<'info,PropertySellProposal>,


    #[account(
        mut,
        seeds =[
                PROPERTY_SYSTEM_SEEDS,
                &seller.property_system_id.to_le_bytes()
        ],
        bump = seller.bump,
    )]
    pub seller:Account<'info,PropertySystemAccount>,

    #[account(
        seeds = [
            b"treasury",
            &seller.key().as_ref(),
        ],
        bump = seller_treasury.bump,
    )]
    
    pub seller_treasury : Account<'info,TreasuryPda>,

    #[account(
        mut,
        seeds=[ 
            PROPERTY_SEED,
            &property_account.property_id.to_le_bytes(),                
            &property_account.state_pubkey.as_ref(),
            &property_account.country_pubkey.as_ref(),                
        ],
        bump = property_account.bump,
        constraint = property_account.property_system == seller.key() @ ErrorCode::InvalidLandForSource
    )]
    pub property_account : Account<'info,PropertyAccount>,

    #[account(
        mut,
        seeds = [
            PROPERTY_PAGE_SEEDS,
            &seller_property_page.page.to_le_bytes(),
            &seller.key().as_ref()
        ],
        bump = seller_property_page.bump,
        constraint = seller_property_page.property_system == seller.key() @ ErrorCode::PropertySystemInvalid
    )]

    pub seller_property_page : Account<'info,PropertyPage>,


}

pub fn execute_buy_proposal(ctx:Context<ExecuteProposal>)->Result<()>{

    let proposal = &mut ctx.accounts.proposal;

    let current_time = Clock::get()?.unix_timestamp;

    // require!(proposal.)


    Ok(())



}