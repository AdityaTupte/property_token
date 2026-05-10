use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token_interface::{Mint, TokenAccount, TokenInterface}};

use crate::{common::{ARBITRAR_REGISTRYSEEDS, DIVIDENDSEEDS, PROPERTY_SYSTEM_SEEDS, REINVESTMENTPDA, SAFETYPDA, THRESHOLD, TREASURYSEEDS, TRUSTEEREGISTRYSEEDS}, errors::ErrorCode, functions::transfer_fro_treasury, state::{ArbitratorRegistry, DividendPda, PropertySystemAccount, ReinvestmentPda, SafetyPda, Threshold, TreasuryPda, TrusteeRegistry,}};

#[derive(Accounts)]
#[instruction(property_system_id:u64)]
pub struct TreasuryDistribution<'info>{


    #[account(mut)]
    pub payer : Signer<'info>,

    #[account(
        seeds=[
            PROPERTY_SYSTEM_SEEDS,
            &property_system_id.to_le_bytes(),
        ],
        bump = property_system.bump
    )]
    pub property_system : Box<Account<'info,PropertySystemAccount>>,

    #[account(
        seeds=[
            THRESHOLD,
            property_system.key().as_ref(),
        ],
        bump = property_system.bump
    )]
    pub thershold : Box<Account<'info,Threshold>>,

    #[account(
        mut,
        seeds=[
            TREASURYSEEDS,
            property_system.key().as_ref(),
        ],
        bump = treasury_pda.bump
    )]
    pub treasury_pda  : Box<Account<'info,TreasuryPda>>,

    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = mint,
        associated_token::authority = treasury_pda,
        associated_token::token_program = token_program,
    )]
    pub treasury_ata : Box<InterfaceAccount<'info,TokenAccount>>,

    #[account(
        mut,
        seeds=[
            DIVIDENDSEEDS,
            property_system.key().as_ref(),
        ],
        bump = dividend_pda.bump)]
    pub dividend_pda : Box<Account<'info,DividendPda>>,

    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = mint,
        associated_token::authority = dividend_pda,
        associated_token::token_program = token_program,
    )]
    pub dividend_ata : Box<InterfaceAccount<'info,TokenAccount>>,

    #[account(
        mut,
    seeds=[
            REINVESTMENTPDA,
            property_system.key().as_ref(),
        ],
        bump = reinvestment_pda.bump)]
    pub reinvestment_pda : Box<Account<'info,ReinvestmentPda>>,

    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = mint,
        associated_token::authority = reinvestment_pda,
        associated_token::token_program = token_program,
    )]
    pub reinvestment_ata : Box<InterfaceAccount<'info,TokenAccount>>,

    #[account(
        mut,
        seeds=[
            SAFETYPDA,
            property_system.key().as_ref(),
        ],
        bump = safety_pda.bump
    )]
    pub safety_pda : Box<Account<'info,SafetyPda>>,

    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = mint,
        associated_token::authority = safety_pda,
        associated_token::token_program = token_program,
    )]
    pub safety_ata : Box<InterfaceAccount<'info,TokenAccount>>,

    #[account(
        mut,
        seeds=[
            TRUSTEEREGISTRYSEEDS,
            property_system.key().as_ref(),
        ],
        bump = trustee_pda.bump
    )]
    pub trustee_pda : Box<Account<'info,TrusteeRegistry>>,

    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = mint,
        associated_token::authority = trustee_pda,
        associated_token::token_program = token_program,
    )]
    pub trustee_ata : Box<InterfaceAccount<'info,TokenAccount>>,

    #[account(
        mut,
        seeds=[
            ARBITRAR_REGISTRYSEEDS,
            property_system.key().as_ref(),
            
        ],
        bump = arbitrar_pda.bump
    )]
    pub arbitrar_pda : Box<Account<'info,ArbitratorRegistry>>, 

    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = mint,
        associated_token::authority = arbitrar_pda,
        associated_token::token_program = token_program,
    )]
    pub arbitrar_ata : Box<InterfaceAccount<'info,TokenAccount>>,

    pub mint : Box<InterfaceAccount<'info,Mint>>,


    pub associated_token_program: Program<'info, AssociatedToken>,

    pub token_program : Interface<'info,TokenInterface>,

    pub system_program :Program<'info,System>,
    

}


pub fn treasury_distribution(
    ctx:Context<TreasuryDistribution>,
    _property_system_id:u64
)->Result<()>{


    let threshold = &ctx.accounts.thershold;

    let treasury = &mut ctx.accounts.treasury_pda;

    let trustee_registry = &mut ctx.accounts.trustee_pda ;

    let arbitrator_registry = &mut ctx.accounts.arbitrar_pda ;

    let now = Clock::get()?.unix_timestamp;

    let valid_time = treasury.last_distribution_ts.checked_add(30*24*60*60).ok_or(ErrorCode::MathOverflow)?;

    require!(now >= valid_time,ErrorCode::DistributionTimeNotReached);

    let bps = (threshold.trustee_salary_threshold as u64).checked_mul(100).ok_or(ErrorCode::MathOverflow)?;

    let treasury_fund = ctx.accounts.treasury_ata.amount;

    let property_sys_key =  ctx.accounts.property_system.key();

    let signer_seeds: &[&[&[u8]]] = &[&[
                TREASURYSEEDS,
                property_sys_key.as_ref(),
                &[treasury.bump],
    ]];

   
    let amount_for_trustee = treasury_fund
                                    .checked_mul(bps as u64)
                                    .ok_or(ErrorCode::MathOverflow)?
                                    .checked_div(10_000)
                                    .ok_or(ErrorCode::MathOverflow)?;

    let bps2 = (threshold.arbitrator_salary_threshold as u64).checked_mul(100).ok_or(ErrorCode::MathOverflow)?;

    let amount_for_arbitrar = treasury_fund
                                    .checked_mul(bps2 as u64)
                                    .ok_or(ErrorCode::MathOverflow)?
                                    .checked_div(10_000)
                                    .ok_or(ErrorCode::MathOverflow)?;

    let bps3 = (threshold.dividend_threshold as u64).checked_mul(100).ok_or(ErrorCode::MathOverflow)?;                            

    let amount_for_dividend = treasury_fund
                                    .checked_mul(bps3 as u64)
                                    .ok_or(ErrorCode::MathOverflow)?
                                    .checked_div(10_000)
                                    .ok_or(ErrorCode::MathOverflow)?;
    
    let bps4 = (threshold.reinvestment_threshold as u64).checked_mul(100).ok_or(ErrorCode::MathOverflow)?;

    let amount_for_reinvestment = treasury_fund
                                    .checked_mul(bps4 as u64)
                                    .ok_or(ErrorCode::MathOverflow)?
                                    .checked_div(10_000)
                                    .ok_or(ErrorCode::MathOverflow)?;

    let bps5 = (threshold.safety_threshold as u64).checked_mul(100).ok_or(ErrorCode::MathOverflow)?;

    let amount_for_safety = treasury_fund
                                    .checked_mul(bps5 as u64)
                                    .ok_or(ErrorCode::MathOverflow)?
                                    .checked_div(10_000)
                                    .ok_or(ErrorCode::MathOverflow)?;


    treasury.last_distribution_ts = now;

   

    let salary_claim_deadline = now.checked_add(30*60*60*24).ok_or(ErrorCode::MathOverflow)?; 

    trustee_registry.claim_deadline_ts = salary_claim_deadline;

    arbitrator_registry.claim_deadline_ts = salary_claim_deadline;

    //for trusteee
   transfer_fro_treasury(
        amount_for_trustee,
        &ctx.accounts.treasury_ata, 
        &ctx.accounts.trustee_ata, 
        &ctx.accounts.mint, 
        treasury, 
        &ctx.accounts.token_program, 
        signer_seeds)?;

    
    //for arbitrar
    transfer_fro_treasury(
        amount_for_arbitrar,
        &ctx.accounts.treasury_ata, 
        &ctx.accounts.arbitrar_ata, 
        &ctx.accounts.mint, 
        treasury, 
        &ctx.accounts.token_program, 
        signer_seeds)?;

    //for reinvestment
    transfer_fro_treasury(
        amount_for_reinvestment,
        &ctx.accounts.treasury_ata, 
        &ctx.accounts.reinvestment_ata, 
        &ctx.accounts.mint, 
        treasury, 
        &ctx.accounts.token_program, 
        signer_seeds)?;

    //for safety
    transfer_fro_treasury(
        amount_for_safety,
        &ctx.accounts.treasury_ata, 
        &ctx.accounts.safety_ata, 
        &ctx.accounts.mint, 
        treasury, 
        &ctx.accounts.token_program, 
        signer_seeds)?;

    //for dividend
   transfer_fro_treasury(
        amount_for_dividend,
        &ctx.accounts.treasury_ata, 
        &ctx.accounts.dividend_ata, 
        &ctx.accounts.mint, 
        treasury, 
        &ctx.accounts.token_program, 
        signer_seeds)?;

    trustee_registry.total_salary_allocated = amount_for_trustee
                                                    .checked_add(ctx.accounts.trustee_ata.amount)
                                                    .ok_or(ErrorCode::MathOverflow)?;

    arbitrator_registry.total_salary_allocated = amount_for_arbitrar
                                                    .checked_add(ctx.accounts.arbitrar_ata.amount)
                                                    .ok_or(ErrorCode::MathOverflow)?;


    Ok(())
}