use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token_interface::{Mint,MintToChecked,mint_to_checked, TokenAccount, TokenInterface}};
use crate::{common::PROPERTY_SYSTEM_SEEDS, state::{ArbitratorRegistry, DividendPda, PropertySystemAccount, ReinvestmentPda, SafetyPda, Threshold, TreasuryPda, TrusteeRegistry,}};
use crate::events::*;
use crate::errors::ErrorCode;

#[derive(Accounts)]
#[instruction(system_id : u64,decimal:u8 , )]
pub struct PropertySystem<'info>{


    #[account(mut)]
    pub creator: Signer<'info>,


    #[account(
        init,
        payer = creator,
        seeds = [ 
            PROPERTY_SYSTEM_SEEDS,
            system_id.to_le_bytes().as_ref(),
        ],
        bump,
        space = 8 + PropertySystemAccount::SIZE
    )]

    pub property_system_acc : Box<Account<'info,PropertySystemAccount>>,


    #[account(
        init,
        payer = creator,
        seeds = [
            b"threshold",
            property_system_acc.key().as_ref()
        ],
        bump,
        space = 8 + Threshold::SIZE,
    )]

    pub threshold : Account<'info,Threshold>,

    #[account(
        init,
        payer = creator,
        seeds = [
            b"treasury",
            property_system_acc.key().as_ref()],
        bump,
        space = 8 + TreasuryPda::SIZE,
    )]

    pub treasury_pda : Box<Account<'info,TreasuryPda>>,


    #[account(
        init,
        payer = creator,
        seeds = [
            b"reinvestment",
            property_system_acc.key().as_ref()
        ],
        bump,
        space = 8 + ReinvestmentPda::SIZE,
    )]

    pub reinvestment_pda : Box<Account<'info,ReinvestmentPda>>,


    #[account(
        init,
        payer = creator,
        seeds = [
            b"safety",
            property_system_acc.key().as_ref()
        ],
        bump,
        space = 8 + SafetyPda::SIZE,
    )]

    pub safety_pda : Box<Account<'info,SafetyPda>>,

     #[account(
        init,
        payer = creator,
        seeds = [
            b"dividend",
            property_system_acc.key().as_ref()
        ],
        bump,
        space = 8 + DividendPda::SIZE,
    )]

    pub dividend_pda :Box<Account<'info,DividendPda>>,


    #[account(
        init,
        payer = creator,
        seeds = [
            b"trustee_registry",
            property_system_acc.key().as_ref()
            ],
        bump,
        space = 8 + TrusteeRegistry::SIZE,
    )]
    pub trustee_registry : Box<Account<'info,TrusteeRegistry>>,

    
    
    #[account(
        init,
        payer = creator,
        seeds = [b"arbitrator_registry",property_system_acc.key().as_ref()],
        bump,
        space = 8 + ArbitratorRegistry::SIZE,
    )]
    pub arbitrator_registry : Box<Account<'info,ArbitratorRegistry>>,



    #[account(
        init,
        payer = creator,
         seeds = [
        b"mint",
        property_system_acc.key().as_ref()
        ],
        bump,
        mint::decimals = decimal,
        mint::authority = property_system_acc.key(),
        mint::freeze_authority = property_system_acc.key(),
    )]
    pub governance_mint: Box<InterfaceAccount<'info, Mint>>,

    // later add metadata program

    #[account(
        init,
        payer = creator,
        associated_token::mint = governance_mint,
        associated_token::authority = creator,
        associated_token::token_program = token_program,
    )]

    pub creator_ata : InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    
    pub system_program: Program<'info, System>,

}

pub fn create(
        ctx:Context<PropertySystem>,
        system_id : u64,decimal:u8,number_of_tokens:u64,
        safety_threshold:u8,
        trustee_salary_threshold:u8,
        arbitrator_salary_threshold:u8,
        dividend_threshold:u8,
        reinvestment_threshold:u8,
    )->Result<()>{

    require_eq!(safety_threshold + trustee_salary_threshold + arbitrator_salary_threshold + dividend_threshold + reinvestment_threshold , 100, ErrorCode::ThresholdInvalid );

    
    let property_system_acc = &mut ctx.accounts.property_system_acc;

    let treasury_pda = &mut ctx.accounts.treasury_pda;

    let trustee_registry = &mut ctx.accounts.trustee_registry;

    let arbitrator_registry = &mut ctx.accounts.arbitrator_registry;

    let governance_mint = &mut ctx.accounts.governance_mint;

    let creator_ata = &mut ctx.accounts.creator_ata;

    let reinvestment_pda = &mut ctx.accounts.reinvestment_pda;

    let safety_pda= &mut ctx.accounts.safety_pda;

    let dividend_pda = &mut ctx.accounts.dividend_pda;

    let threshold=&mut ctx.accounts.threshold;

    
    

    
    let signer_seeds: &[&[&[u8]]] = &[&[
        b"property_system_account".as_ref(),
        &system_id.to_le_bytes(),
        &[ctx.bumps.property_system_acc]]];
    
    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        MintToChecked{
            mint:governance_mint.to_account_info(),
            to:creator_ata.to_account_info(),   
            authority:property_system_acc.to_account_info()
        },
        &signer_seeds,
    );
    

    let current_time = Clock::get()?.unix_timestamp;

    property_system_acc.property_system_id = system_id;

    property_system_acc.governance_mint = governance_mint.key();

    property_system_acc.treasury = treasury_pda.key();

    property_system_acc.trustee_registry = trustee_registry.key();

    property_system_acc.arbitrator_registry = arbitrator_registry.key();

    property_system_acc.total_properties = 0;

    property_system_acc.max_page = 0;

    property_system_acc.created_at = current_time;

    property_system_acc.total_token_supply = number_of_tokens;

    property_system_acc.creator = ctx.accounts.creator.key();

    property_system_acc.bump = ctx.bumps.property_system_acc;

    //threshold

    threshold.trustee_salary_threshold = trustee_salary_threshold;

    threshold.arbitrator_salary_threshold = arbitrator_salary_threshold;

    threshold.dividend_threshold = dividend_threshold;

    threshold.reinvestment_threshold = reinvestment_threshold;

    threshold.safety_threshold = safety_threshold;

    //treasury_pda

    treasury_pda.property_system_accout = property_system_acc.key();

    treasury_pda.bump = ctx.bumps.treasury_pda;

    //reinvestment_pda

    reinvestment_pda.property_system = property_system_acc.key();

    reinvestment_pda.bump = ctx.bumps.reinvestment_pda;

    reinvestment_pda.reinvestement_used = 0;


    //safety

    safety_pda.property_system = property_system_acc.key();

    safety_pda.bump = ctx.bumps.safety_pda;

    //dividend

    dividend_pda.property_system = property_system_acc.key();

    dividend_pda.bump = ctx.bumps.dividend_pda;


    // trusteeregistry

    trustee_registry.property_system_account = property_system_acc.key();

    trustee_registry.bump = ctx.bumps.trustee_registry;


    //arbitrator_registry

    arbitrator_registry.property_system_account = property_system_acc.key();

    arbitrator_registry.bump = ctx.bumps.arbitrator_registry;

    mint_to_checked(cpi_ctx,number_of_tokens,decimal).map_err(|_| ErrorCode::MintFailed)?;

    emit!(  PropertySystemCreated {
    property_system: property_system_acc.key(),
    creator: ctx.accounts.creator.key(),
    governance_mint: governance_mint.key(),

    treasury: treasury_pda.key(),
    reinvestment: reinvestment_pda.key(),
    safety: safety_pda.key(),
    dividend: dividend_pda.key(),

    safety_threshold,
    trustee_salary_threshold,
    arbitrator_salary_threshold,
    dividend_threshold,
    reinvestment_threshold,

    created_at: current_time,

    });


    Ok(())

}