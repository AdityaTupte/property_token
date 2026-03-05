use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken,token_interface::{Mint, TokenAccount, TokenInterface,TransferChecked, transfer_checked}};

use crate::{constant::{BUYPROPERTY, HARDCODED_PUBKEY, PROPERTY_PAGE_SEEDS, PROPERTY_SEED, PROPERTY_SYSTEM_SEEDS, ProposalStatus, REINVESTMENTPDA, SELLPROPERTY}, state::{PropertyAccount, PropertyBuyProposal, PropertyPage, PropertySellProposal, PropertySystemAccount, ReinvestmentPda, TreasuryPda, TrusteeRegistry}};

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
        mut,
        seeds = [
            PROPERTY_PAGE_SEEDS,
            &buyer_property_page.page.to_le_bytes(),
            &buyer.key().as_ref()
        ],
        bump = buyer_property_page.bump,
        constraint = buyer_property_page.property_system == buyer.key() @ ErrorCode::PropertySystemInvalid
    )]

    pub buyer_property_page : Account<'info,PropertyPage>,

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
        constraint = proposal.status == ProposalStatus::Passed @ ErrorCode::ProposalNotPassed,
        constraint = proposal.property == property_account.key() @ ErrorCode::InvalidProperty,
        constraint = proposal.buyer == buyer.key() @ ErrorCode::InvalidProposal,
        constraint = proposal.buyer_wallet == buyer_wallet.key() @ ErrorCode::InvalidReinvestAccount,
        constraint = proposal.sell_proposal == sell_proposal.key() @ ErrorCode::InvalidSellProposal,
        )]
    pub proposal : Account<'info,PropertyBuyProposal>,

    #[account(
        associated_token::mint = HARDCODED_PUBKEY,
        associated_token::authority = buyer_wallet ,
    )]
    pub buyer_ata : InterfaceAccount<'info, TokenAccount>,

    
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
        constraint = sell_proposal.status == ProposalStatus::Passed @ ErrorCode::ProposalNotPassed,
        constraint = sell_proposal.property_system_account  == seller.key() @ ErrorCode::InvalidProposal,
        constraint = sell_proposal.deposit_account_pda == seller_treasury.key() @ ErrorCode::InvalidTreasury,
        constraint = sell_proposal.property_account == property_account.key() @ ErrorCode::InvalidProperty 
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
        constraint = seller.treasury == seller_treasury.key() @ ErrorCode:: InvalidTreasury
    )]
    
    pub seller_treasury : Account<'info,TreasuryPda>,

     #[account(
        associated_token::mint = HARDCODED_PUBKEY,
        associated_token::authority = seller_treasury ,
    )]
    pub seller_ata : InterfaceAccount<'info, TokenAccount>,

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

    #[account(
        address = HARDCODED_PUBKEY,
    )]
    pub mint : InterfaceAccount<'info,Mint>,

    pub token_program : Interface<'info,TokenInterface>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    
    pub system_program: Program<'info, System>,


}

pub fn execute_buy_proposal(ctx:Context<ExecuteProposal>)->Result<()>{

    let proposal = &mut ctx.accounts.proposal;

    let sell_proposal = &mut ctx.accounts.sell_proposal;

    let sell_property_page = &mut ctx.accounts.seller_property_page;

    let property = &mut ctx.accounts.property_account;

    let buy_property_page = &mut ctx.accounts.buyer_property_page;

    let buyer = &mut ctx.accounts.buyer;

    let seller = &mut ctx.accounts.seller;

    let buyer_key =  buyer.key();
    
    let current_time = Clock::get()?.unix_timestamp;

    require!(sell_property_page.land.contains(&property.key()), ErrorCode::InvalidProperty);
    
    require!(buy_property_page.land.len() < 100, ErrorCode::InsufficentSpace);

    require!(current_time <= sell_proposal.transfer_deadline  && current_time <= proposal.payment_deadline , ErrorCode::CantTramnsfer );

    require!(seller.key() != buyer.key(),ErrorCode::SamePropertySystem);

    require!(proposal.sale_price == sell_proposal.sale_price ,ErrorCode::DiffrentPrice);
    
    let amount = proposal.sale_price;

    let decimals = ctx.accounts.mint.decimals;


    

   if  let Some(index) = sell_property_page.land.iter().position(|x| *x == property.key()){

        sell_property_page.land.swap_remove(index);

   }

   buy_property_page.land.push(property.key());

   property.property_system = buyer.key();
   
   property.page_number = buy_property_page.page;

   let cpi_accounts = TransferChecked{
    from:ctx.accounts.buyer_ata.to_account_info(),
    to : ctx.accounts.seller_ata.to_account_info(),
    authority:ctx.accounts.buyer_wallet.to_account_info(),
    mint:ctx.accounts.mint.to_account_info(),
   };
   
    let signer_seed:&[&[&[u8]]] = &[&[
        REINVESTMENTPDA,
        buyer_key.as_ref(),
        &[ctx.accounts.buyer_wallet.bump]]];

   let cpi_program = ctx.accounts.token_program.to_account_info(); 

   let cpi_context = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seed);

   transfer_checked(cpi_context, amount, decimals)?;

    Ok(())



}