use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token_interface::{Mint,TransferChecked, transfer_checked, TokenAccount, TokenInterface}};

use crate::{common::{ARBITRAR_RECEIPT_SEEDS, ARBITRAR_REGISTRYSEEDS, ARBITRAR_SALARY, HARDCODED_PUBKEY, PROPERTY_SYSTEM_SEEDS, TRUSTEEREGISTRYSEEDS}, errors::ErrorCode,  state::{ArbitratorRecepit, ArbitratorRegistry, PropertySystemAccount, SalaryPda, arbitrator_recepit }};

#[derive(Accounts)]
#[instruction(property_system_id:u64)]
pub struct ClaimArbitrarSalary<'info>{

#[account(
    seeds=[
        PROPERTY_SYSTEM_SEEDS,
        &property_system_id.to_le_bytes()
    ],
    bump= property_system.bump
)]
pub property_system : Account<'info,PropertySystemAccount>, 

#[account(
    mut,

)]
pub signer:Signer<'info>,

#[account()]
pub arbitrar:SystemAccount<'info>,

#[account(
    mut,
        seeds = [
            ARBITRAR_RECEIPT_SEEDS,
            property_system.key().as_ref(),
            arbitrar.key().as_ref()
        ],
        bump = arbitirar_receipt.bump,
    )]
    pub arbitirar_receipt: Account<'info,ArbitratorRecepit>,


#[account(
    init_if_needed,
    payer=signer,
    associated_token::mint= mint,
    associated_token::authority = arbitrar, 
    associated_token::token_program = token_program
)]
pub arbitrar_ata : InterfaceAccount<'info,TokenAccount>,

// #[account(
//     init_if_needed,
//     payer = arbitrar_registry,
//     seeds=[
//             ARBITRAR_SALARY,
//             property_system.key().as_ref()
//     ],
//     bump,
//     space = 8 + SalaryPda::SIZE
// )]
// pub arbitrar_salary_pda : Account<'info,SalaryPda>,

#[account(
    mut,
    seeds=[
        ARBITRAR_REGISTRYSEEDS,
        property_system.key().as_ref()
    ],
    bump = arbitrar_registry.bump,
    //constraint = arbitrar_registry.arbitrator.contains(&arbitrar.key()) @ ErrorCode::NotAuthorized
)]
pub arbitrar_registry : Account<'info,ArbitratorRegistry>,

#[account(
    mut,
    associated_token::mint = mint,
    associated_token::authority = arbitrar_registry, 
    associated_token::token_program = token_program
)]
pub arbitrar_registry_ata : InterfaceAccount<'info,TokenAccount>,

pub system_program : Program<'info,System>,

#[account(
    // address = HARDCODED_PUBKEY
)]
pub mint : InterfaceAccount<'info,Mint>,

pub associated_token_program: Program<'info, AssociatedToken>,

pub token_program : Interface<'info,TokenInterface>,


}


pub fn claim_arbitrar_salary(
    ctx:Context<ClaimArbitrarSalary>,
    _property_system_id:u64
)->Result<()>{

    let arbitrar_registry_pda = &mut ctx.accounts.arbitrar_registry;

    let arbitrator_recepit = &mut ctx.accounts.arbitirar_receipt;

    require!(arbitrar_registry_pda.claim_deadline_ts > arbitrator_recepit.new_transaction_time,ErrorCode::DeadlineReached);

    if arbitrator_recepit.new_transaction_time != 0 as i64  {
        let salary = arbitrar_registry_pda.total_salary_allocated
                                            .checked_div(arbitrar_registry_pda.total_arbitrators as u64)
                                            .ok_or(ErrorCode::MathOverflow)?;

    arbitrator_recepit.new_transaction_time = arbitrar_registry_pda.claim_deadline_ts;

    let property_sys_key =   ctx.accounts.property_system.key();

    let signer_seeds: &[&[&[u8]]] = &[&[
                ARBITRAR_REGISTRYSEEDS,
                property_sys_key.as_ref(),
                &[ctx.accounts.arbitrar_registry.bump]
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

    }

    else {
         arbitrator_recepit.new_transaction_time = arbitrar_registry_pda.claim_deadline_ts;
    }
    Ok(())


}