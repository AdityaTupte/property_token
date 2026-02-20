use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token_interface::{TokenAccount, TokenInterface}};

use crate::{constant::{BUYPROPOSAL, HARDCODED_PUBKEY, LAND_PAGE_SEEDS, LAND_SEED, PROPERTY_SYSTEM_SEEDS, ProposalStatus}, errors::ErrorCode, state::{BuyLandProposalDetail, LandAccount, LandPage, PropertySystemAccount, ReinvestmentPda, TreasuryPda, TrusteeRegistry}};


#[derive(Accounts)]
#[instruction(proposal_id:u64)]
pub struct BuyProposal<'info>{

    #[account(
        mut,
        constraint = trustee_registry.trustees.contains(&trustee.key()) @ ErrorCode::NotAuthorized
    )]
    pub trustee: Signer<'info>,


    #[account(
        init,
        payer = trustee,
        seeds = [
            BUYPROPOSAL,
            buyer_property_system.key().as_ref(),
            &proposal_id.to_le_bytes(),
        ],
        bump,
        space = BuyLandProposalDetail::SIZE
    )]
    pub buy_proposal:Account<'info,BuyLandProposalDetail>,

    #[account(
        constraint = buyer_property_system.trustee_registry == trustee_registry.key() @ ErrorCode::InvalidTrusteeRegsitry
    )]
    pub trustee_registry : Account<'info,TrusteeRegistry>,

    #[account(
        seeds=[
            PROPERTY_SYSTEM_SEEDS,
            &buyer_property_system.property_system_id.to_le_bytes()
        ],
        bump = buyer_property_system.bump
    )]
    pub buyer_property_system : Account<'info,PropertySystemAccount>,

    #[account(
        constraint = buyer_property_system.treasury == buyer_treasury_pda.key() @ ErrorCode::InvalidTreasury
    )]
    pub buyer_treasury_pda : Account<'info,TreasuryPda>,

    #[account(
        constraint = buyer_treasury_pda.reinvenstement_acc == buyer_reinvestment_pda.key() @ ErrorCode::InvalidReinvestAccount
    )]
    pub buyer_reinvestment_pda : Account<'info,ReinvestmentPda>,

    #[account(
        associated_token::mint = HARDCODED_PUBKEY ,
        associated_token::authority = buyer_reinvestment_pda,
    )]
    pub buyer_reinvestment_ata : InterfaceAccount<'info,TokenAccount>,
    
    ///////////////////////////
    
    #[account(
        seeds=[
            PROPERTY_SYSTEM_SEEDS,
            &seller_property_system.property_system_id.to_le_bytes()
        ],
        bump = seller_property_system.bump
    )]
    pub seller_property_system : Account<'info,PropertySystemAccount>,

    #[account(
        constraint = seller_property_system.treasury == seller_treasury_pda.key() @ ErrorCode::InvalidTreasury
    )]
    pub seller_treasury_pda : Account<'info,TreasuryPda>,


    #[account(
        associated_token::mint = HARDCODED_PUBKEY ,
        associated_token::authority = seller_treasury_pda,
    )]
    pub seller_treasury_ata : InterfaceAccount<'info,TokenAccount>,
    
    #[account(
        seeds=[
            LAND_SEED,
            &land_account.land_id.to_le_bytes(),
            land_account.state_pubkey.as_ref(),
            land_account.country_pubkey.as_ref()
        ],
        bump = land_account.bump,
        constraint = land_account.property_system == seller_property_system.key() @ ErrorCode::InvalidLand
    )]
    pub land_account : Account<'info,LandAccount>,

    #[account(
        seeds=[
            LAND_PAGE_SEEDS,
            &seller_land_page.page.to_le_bytes(),
            seller_property_system.key().as_ref()
        ],
        bump = seller_land_page.bump,
        constraint = seller_land_page.land.contains(&land_account.key()) @ ErrorCode::LandPageInvalid
    )]
    pub seller_land_page : Account<'info,LandPage>,

    
    pub token_program: Interface<'info, TokenInterface>,
    
    pub associated_token_program: Program<'info, AssociatedToken>,

    pub system_program : Program<'info,System>,

}


pub fn create_buy_proposal(
    ctx:Context<BuyProposal>,
    proposal_id : u64,
    amount_to_transfer:u64,
)->Result<()>{

    let buy_proposal = &mut ctx.accounts.buy_proposal;

    let buyer_property_system = & ctx.accounts.buyer_property_system;

    let buyer_reinvestment_ata = &ctx.accounts.buyer_reinvestment_ata;

    let seller_property_system = &ctx.accounts.seller_property_system;

    let seller_treasury_ata = &ctx.accounts.seller_treasury_ata;

    let land_account = &ctx.accounts.land_account;

    require!(seller_property_system.key() != buyer_property_system.key() , ErrorCode::SamePropertySystem );


    buy_proposal.proposal_id = proposal_id;

    buy_proposal.land = land_account.key();

    buy_proposal.amount_to_transfer = amount_to_transfer;

    buy_proposal.buyer_property_system = buyer_property_system.key();

    buy_proposal.buyer_reinvestment = buyer_reinvestment_ata.key();

    buy_proposal.seller_property_system = seller_property_system.key();

    buy_proposal.seller_treasury = seller_treasury_ata.key();

    buy_proposal.proposal_status = ProposalStatus::Active as u8;

    buy_proposal.bump = ctx.bumps.buy_proposal;

    Ok(())



}