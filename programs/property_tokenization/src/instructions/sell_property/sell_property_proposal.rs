use anchor_lang::prelude::*;
use crate::constant::*;
use crate::errors::ErrorCode;
use crate::state::{PropertyAccount, PropertyPage, PropertySellProposal, PropertySystemAccount, TreasuryPda, TrusteeRegistry};


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
        space = PropertySellProposal::SIZE
    )]

    pub proposal : Account<'info,PropertySellProposal>,

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
            PROPERTY_SEED,
            &property_account.property_id.to_le_bytes(),                
            &property_account.state_pubkey.as_ref(),
            &property_account.country_pubkey.as_ref(),                
        ],bump = property_account.bump,
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

    pub system_program: Program<'info,System>,

}


pub fn create_sell_proposal(ctx:Context<SellLandProposal>,proposal_id: u64,sale_price:u64,)->Result<()>{

    let seller = &ctx.accounts.seller ;

    let seller_land_page = &ctx.accounts.seller_property_page;

    let property_account = & ctx.accounts.property_account ;

    let seller_treasury = & ctx.accounts.seller_treasury;

    let proposal =&mut ctx.accounts.proposal;

    require!(seller_land_page.land.contains(&property_account.key()), ErrorCode::InvalidLand);

    proposal.initialize(
    proposal_id,
    property_account.key(),
    seller.key(),
    seller_treasury.key(),
    sale_price,
    seller.total_token_supply,
    
    );
    
    Ok(())


}