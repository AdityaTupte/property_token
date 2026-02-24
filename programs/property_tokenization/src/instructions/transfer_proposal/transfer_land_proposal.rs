use anchor_lang::prelude::*;
use crate::constant::*;
use crate::errors::ErrorCode;
use crate::state::{LandAccount, LandPage, PropertySystemAccount, ReinvestmentPda, TransferLandDetail, TreasuryPda, TrusteeRegistry};


#[derive(Accounts)]
#[instruction(proposal_id : u64)]
pub struct SellLandProposal<'info>{

    #[account(
        init,
        payer = signer,
        seeds =[
            SELLPROPERTY,
            seller.key().as_ref(),
            &proposal_id.to_le_bytes(),
        ],
        bump,
        space = TransferLandDetail::SIZE
    )]

    pub proposal : Account<'info,TransferLandDetail>,

    #[account(
        mut,
        constraint = trustee_registry.trustees.contains(&signer.key()) @ ErrorCode::NotAuthorized
    )]

    pub signer:Signer<'info>,

    #[account(
        seeds =[
                PROPERTY_SYSTEM_SEEDS,
                &seller.property_system_id.to_le_bytes()
        ],
        bump = seller.bump,
    )]
    pub seller:Account<'info,PropertySystemAccount>,

    #[account(
        constraint = seller.trustee_registry == trustee_registry.key() @ ErrorCode::InvalidTrusteeRegsitry
    )]
    pub trustee_registry: Account<'info,TrusteeRegistry>,

    #[account(
        seeds = [
            b"treasury",
            &seller.key().as_ref(),
        ],
        bump = seller_treasury.bump
    )]
    
    pub seller_treasury : Account<'info,TreasuryPda>,


    #[account(
        seeds=[ 
            LAND_SEED,
            &land_account.land_id.to_le_bytes(),                
            &land_account.state_pubkey.as_ref(),
            &land_account.country_pubkey.as_ref(),                
        ],bump = land_account.bump,
        constraint = land_account.property_system == seller.key() @ ErrorCode::InvalidLandForSource
    )]
    pub land_account : Account<'info,LandAccount>,


   #[account(
        mut,
        seeds = [
            LAND_PAGE_SEEDS,
            &seller_landpage.page.to_le_bytes(),
            &seller.key().as_ref()
        ],
        bump = seller_landpage.bump,
        constraint = seller_landpage.property_system == seller.key() @ ErrorCode::PropertySystemInvalid
    )]

    pub seller_landpage : Account<'info,LandPage>,


     #[account(
        seeds =[
                PROPERTY_SYSTEM_SEEDS,
                &buyer.property_system_id.to_le_bytes()
        ],
        bump = buyer.bump,
    )]
    pub buyer:Account<'info,PropertySystemAccount>,

    #[account(
        seeds = [
            b"treasury",
            &buyer.key().as_ref(),
        ],
        bump = buyer_treasury.bump,
        constraint = buyer.treasury == buyer_treasury.key() @ ErrorCode::InvalidTreasury
    )]
    
    pub buyer_treasury : Account<'info,TreasuryPda>,

    #[account(
        seeds=[
            b"reinvestment",
            buyer.key().as_ref()
        ],
        bump = buyer_reinvestment.bump,
        constraint = buyer_treasury.reinvenstement_acc == buyer_reinvestment.key() @ ErrorCode::InvalidReinvestAccount
    )]
    pub buyer_reinvestment : Account<'info,ReinvestmentPda>,


    pub system_program: Program<'info,System>,

}


pub fn transfer_proposal(ctx:Context<SellLandProposal>,proposal_id: u64,amount:u64)->Result<()>{

    let seller = &ctx.accounts.seller ;

    let seller_land_page = &ctx.accounts.seller_landpage;

    let land_account = & ctx.accounts.land_account ;

    let seller_treasury = & ctx.accounts.seller_treasury;

    let proposal = &mut ctx.accounts.proposal;

    let buyer = & ctx.accounts.buyer;

    let buyer_reinvestment = &ctx.accounts.buyer_reinvestment;

    require!( seller.key() != buyer.key(), ErrorCode::SamePropertySystem);

    require!(seller_land_page.land.contains(&land_account.key()), ErrorCode::InvalidLand);

    proposal.initialize(
    proposal_id,
    land_account.key(),
    seller.key(),
    seller_treasury.key(),
    buyer.key(),
    buyer_reinvestment.key(),
    amount,
    ProposalType::SELLPROPERTY, 
    );
    proposal.total_voting_power = seller.total_token_supply;
    
    Ok(())


}