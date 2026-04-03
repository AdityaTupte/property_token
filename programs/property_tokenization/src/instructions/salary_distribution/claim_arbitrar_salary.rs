use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token_interface::{Mint,TransferChecked, transfer_checked, TokenAccount, TokenInterface}};

use crate::{common::{ARBITRAR_REGISTRYSEEDS, ARBITRAR_SALARY, HARDCODED_PUBKEY, PROPERTY_SYSTEM_SEEDS, TRUSTEEREGISTRYSEEDS}, errors::ErrorCode,  state::{ArbitratorRegistry, PropertySystemAccount, SalaryPda, }};

#[derive(Accounts)]

pub struct ClaimArbitrarSalary<'info>{

#[account(
    seeds=[
        PROPERTY_SYSTEM_SEEDS,
        &property_system.property_system_id.to_be_bytes()
    ],
    bump= property_system.bump
)]
pub property_system : Account<'info,PropertySystemAccount>, 

#[account()]
pub arbitrar:SystemAccount<'info>,

#[account(
    mut,
    associated_token::mint= mint,
    associated_token::authority = arbitrar, 
)]
pub arbitrar_ata : InterfaceAccount<'info,TokenAccount>,

#[account(
    init_if_needed,
    payer = arbitrar_registry,
    seeds=[
            ARBITRAR_SALARY,
            property_system.key().as_ref()
    ],
    bump,
    space = 8 + SalaryPda::SIZE
)]
pub arbitrar_salary_pda : Account<'info,SalaryPda>,

#[account(
    mut,
    seeds=[
        ARBITRAR_REGISTRYSEEDS,
        property_system.key().as_ref()
    ],
    bump = arbitrar_registry.bump,
    constraint = arbitrar_registry.arbitrator.contains(&arbitrar.key()) @ ErrorCode::NotAuthorized
)]
pub arbitrar_registry : Account<'info,ArbitratorRegistry>,

#[account(
    mut,
    associated_token::mint = mint,
    associated_token::authority = arbitrar_registry, 
)]
pub arbitrar_registry_ata : InterfaceAccount<'info,TokenAccount>,

pub system_program : Program<'info,System>,

#[account(
    address = HARDCODED_PUBKEY
)]
pub mint : InterfaceAccount<'info,Mint>,

pub associated_token_program: Program<'info, AssociatedToken>,

pub token_program : Interface<'info,TokenInterface>,


}


pub fn claim_trustee_salary(
    ctx:Context<ClaimArbitrarSalary>
)->Result<()>{

    let arbitrar_registry_pda = &mut ctx.accounts.arbitrar_registry;

    let arbitrar_salary_pda = &mut ctx.accounts.arbitrar_salary_pda;

    require!(arbitrar_registry_pda.claim_deadline_ts > arbitrar_salary_pda.new_transaction_time,ErrorCode::DeadlineReached);

    let salary = ctx.accounts.arbitrar_registry_ata.amount
                                            .checked_div(arbitrar_registry_pda.arbitrator.len() as u64)
                                            .ok_or(ErrorCode::MathOverflow)?;

    arbitrar_salary_pda.new_transaction_time = arbitrar_registry_pda.claim_deadline_ts;

    let property_sys_key =   ctx.accounts.property_system.key();

    let signer_seeds: &[&[&[u8]]] = &[&[
                TRUSTEEREGISTRYSEEDS,
                property_sys_key.as_ref()
    ]];
    
    let cpi_accounts = TransferChecked{
        from:ctx.accounts.arbitrar_registry_ata.to_account_info(),
        mint: ctx.accounts.mint.to_account_info(),
        to:ctx.accounts.arbitrar_ata.to_account_info(),
        authority:ctx.accounts.arbitrar_registry.to_account_info(),
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