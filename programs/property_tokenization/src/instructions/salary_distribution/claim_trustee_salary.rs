use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token_interface::{Mint,TransferChecked, transfer_checked, TokenAccount, TokenInterface}};

use crate::{common::{HARDCODED_PUBKEY, PROPERTY_SYSTEM_SEEDS, TRUSTEE_SALARY, TRUSTEEREGISTRYSEEDS}, errors::ErrorCode,  state::{PropertySystemAccount, SalaryPda, TrusteeRegistry}};

#[derive(Accounts)]

pub struct ClaimTrusteeSalary<'info>{

#[account(
    seeds=[
        PROPERTY_SYSTEM_SEEDS,
        &property_system.property_system_id.to_be_bytes()
    ],
    bump= property_system.bump
)]
pub property_system : Account<'info,PropertySystemAccount>, 

#[account()]
pub trustee:SystemAccount<'info>,

#[account(
    associated_token::mint= mint,
    associated_token::authority = trustee, 
)]
pub trustee_ata : InterfaceAccount<'info,TokenAccount>,

#[account(
    init_if_needed,
    payer = trustee_registry_ata,
    seeds=[
            TRUSTEE_SALARY,
            property_system.key().as_ref()
    ],
    bump,
    space = 8 + SalaryPda::SIZE
)]
pub trustee_salary_pda : Account<'info,SalaryPda>,

#[account(
    seeds=[
        TRUSTEEREGISTRYSEEDS,
        property_system.key().as_ref()
    ],
    bump = trustee_registry.bump,
    constraint = trustee_registry.trustees.contains(&trustee.key()) @ ErrorCode::NotAuthorized
)]
pub trustee_registry : Account<'info,TrusteeRegistry>,

#[account(
    mut,
    associated_token::mint = mint,
    associated_token::authority = trustee, 
)]
pub trustee_registry_ata : InterfaceAccount<'info,TokenAccount>,

pub system_program : Program<'info,System>,

#[account(
    address = HARDCODED_PUBKEY
)]
pub mint : InterfaceAccount<'info,Mint>,

pub associated_token_program: Program<'info, AssociatedToken>,

pub token_program : Interface<'info,TokenInterface>,


}


pub fn claim_trustee_salary(
    ctx:Context<ClaimTrusteeSalary>
)->Result<()>{

    let trustee_registry_pda = &mut ctx.accounts.trustee_registry;

    let trustee_salary_pda = &mut ctx.accounts.trustee_salary_pda;

    require!(trustee_registry_pda.claim_deadline_ts > trustee_salary_pda.new_transaction_time,ErrorCode::DeadlineReached);

    let salary = ctx.accounts.trustee_registry_ata.amount
                                            .checked_div(trustee_registry_pda.trustees.len() as u64)
                                            .ok_or(ErrorCode::MathOverflow)?;

    trustee_salary_pda.new_transaction_time = trustee_registry_pda.claim_deadline_ts;

    let property_sys_key =   ctx.accounts.property_system.key();

    let signer_seeds: &[&[&[u8]]] = &[&[
                TRUSTEEREGISTRYSEEDS,
                property_sys_key.as_ref()
    ]];
    
    let cpi_accounts = TransferChecked{
        from:ctx.accounts.trustee_registry_ata.to_account_info(),
        mint: ctx.accounts.mint.to_account_info(),
        to:ctx.accounts.trustee_ata.to_account_info(),
        authority:ctx.accounts.trustee_registry.to_account_info(),
    };

    let ctx1 = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        cpi_accounts,
        signer_seeds
    );

    let decimal = ctx.accounts.mint.decimals;

    transfer_checked(ctx1,salary, decimal)?;

    Ok(())


}