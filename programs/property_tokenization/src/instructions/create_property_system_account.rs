use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token_interface::{Mint,MintToChecked,mint_to_checked, TokenAccount, TokenInterface}};
use crate::state::{ArbitratorRegistry, DividendPda, PropertySystemAccount, PropertySystemCounter, ReinvestmentPda, SafetyPda, Threshold, TreasuryPda, TrusteeRegistry, threshold};

const HARDCODED_PUBKEY: Pubkey = pubkey!("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");
#[derive(Accounts)]
#[instruction(decimal:u8)]
pub struct PropertySystemAcc<'info>{


    #[account(mut)]
    pub creator: Signer<'info>,

    #[account(
        mut,
        seeds = [b"property_system_count"],
        bump
    )]
    pub property_system_count : Account<'info,PropertySystemCounter>,

    #[account(
        init,
        payer = creator,
        seeds = [ 
            b"property_system_account".as_ref(),
            &(property_system_count.total_property_system + 1).to_le_bytes()],
        bump,
        space = 8 + PropertySystemAccount::SIZE
    )]

    pub property_system_acc : Account<'info,PropertySystemAccount>,

    #[account(
        address = HARDCODED_PUBKEY,
        
    )]

    pub stable_coin_mint : InterfaceAccount<'info, Mint>,

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

    pub treasury_pda : Account<'info,TreasuryPda>,



    #[account(
        init,
        payer = creator,
        associated_token::mint = stable_coin_mint,
        associated_token::authority = treasury_pda,
        associated_token::token_program = token_program,
    )]

    pub treasury_pda_ata : InterfaceAccount<'info,TokenAccount>,


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

    pub reinvestment_pda : Account<'info,ReinvestmentPda>,

    #[account(
        init,
        payer = creator,
        associated_token::mint = stable_coin_mint,
        associated_token::authority = reinvestment_pda,
        associated_token::token_program = token_program,
    )]

    pub reinvestment_pda_ata : InterfaceAccount<'info,TokenAccount>,

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

    pub safety_pda : Account<'info,SafetyPda>,

     #[account(
        init,
        payer = creator,
        associated_token::mint = stable_coin_mint,
        associated_token::authority = safety_pda,
        associated_token::token_program = token_program,
    )]

    pub safety_pda_ata : InterfaceAccount<'info,TokenAccount>,


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

    pub dividend_pda : Account<'info,DividendPda>,

     #[account(
        init,
        payer = creator,
        associated_token::mint = stable_coin_mint,
        associated_token::authority = dividend_pda,
        associated_token::token_program = token_program,
    )]

    pub dividend_pda_ata : InterfaceAccount<'info,TokenAccount>,


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
    pub trustee_registry : Account<'info,TrusteeRegistry>,

    #[account(
        init,
        payer = creator,
        associated_token::mint = stable_coin_mint,
        associated_token::authority = treasury_pda,
        associated_token::token_program = token_program,
    )]

    pub trustee_pda_ata : InterfaceAccount<'info,TokenAccount>,
    
    
    #[account(
        init,
        payer = creator,
        seeds = [b"arbitrator_registry",property_system_acc.key().as_ref()],
        bump,
        space = 8 + ArbitratorRegistry::SIZE,
    )]
    pub arbitrator_registry : Account<'info,ArbitratorRegistry>,

    #[account(
        init,
        payer = creator,
        associated_token::mint = stable_coin_mint,
        associated_token::authority = arbitrator_registry,
        associated_token::token_program = token_program,
    )]

    pub arbirtrator_pda_ata : InterfaceAccount<'info,TokenAccount>,


    #[account(
        init,
        payer = creator,
        mint::decimals = decimal,
        mint::authority = property_system_acc.key(),
        mint::freeze_authority = property_system_acc.key(),
    )]
    pub governance_mint: InterfaceAccount<'info, Mint>,

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

pub fn create_property_system_account(
        ctx:Context<PropertySystemAcc>,
        decimal:u8,amount:u64,
        safety_threshold:u8,
        trustee_salary_threshold:u8,
        arbitrator_salary_threshold:u8,
        dividend_threshold:u8,
        reinvestment_threshold:u8
    )->Result<()>{

    require_eq!(safety_threshold + trustee_salary_threshold + arbitrator_salary_threshold + dividend_threshold + reinvestment_threshold , 100 );

    let property_system_count = &mut ctx.accounts.property_system_count;

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
        b"property-system-account".as_ref(),
        &(property_system_count.total_property_system + 1).to_le_bytes(),
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
    
    mint_to_checked(cpi_ctx,amount,decimal)?;

    let current_time = Clock::get()?.unix_timestamp;

    property_system_count.total_property_system  += 1;

    property_system_acc.governance_mint = governance_mint.key();

    property_system_acc.treasury = treasury_pda.key();

    property_system_acc.trustee_registry = trustee_registry.key();

    property_system_acc.arbitrator_registry = arbitrator_registry.key();

    property_system_acc.total_properties = 0;

    property_system_acc.max_page = 0;

    property_system_acc.created_at = current_time;

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

    treasury_pda.reinvenstement_acc = reinvestment_pda.key() ;

    treasury_pda.safety_acc = safety_pda.key() ;

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

    trustee_registry.property_system_accout = property_system_acc.key();

    trustee_registry.bump = ctx.bumps.trustee_registry;


    //arbitrator_registry

    arbitrator_registry.property_system_accout = property_system_acc.key();

    arbitrator_registry.bump = ctx.bumps.arbitrator_registry;


    Ok(())

}