use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token_interface::{Mint,TransferChecked, transfer_checked, TokenAccount, TokenInterface}};

use crate::{common::{HARDCODED_PUBKEY, PROPERTY_SYSTEM_SEEDS, TRUSTEE_RECEIPT_SEEDS,  TRUSTEEREGISTRYSEEDS}, errors::ErrorCode,  state::{PropertySystemAccount,  TrusteeRecepit, TrusteeRegistry}};

use crate::functions::check_property_system;
#[derive(Accounts)]
#[instruction(property_system_id:u64)]
pub struct ClaimTrusteeSalary<'info>{

pub associated_token_program: Program<'info, AssociatedToken>, 

#[account(
    // address = HARDCODED_PUBKEY
)]
pub mint : InterfaceAccount<'info,Mint>,

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



// pub system_program : Program<'info,System>,

// #[account(
//     init_if_needed,
//     payer = trustee_registry_ata,
//     seeds=[
//             TRUSTEE_SALARY,
//             property_system.key().as_ref()
//     ],
//     bump,
//     space = 8 + SalaryPda::SIZE
// )]
// pub trustee_salary_pda : Account<'info,SalaryPda>,

pub token_program : Interface<'info,TokenInterface>,

#[account()]
pub trustee:SystemAccount<'info>,

#[account(
    init_if_needed,
    payer=signer,
    associated_token::mint= mint,
    associated_token::authority = trustee, 
    associated_token::token_program = token_program
)]
pub trustee_ata : InterfaceAccount<'info,TokenAccount>,

#[account(
    mut,
        seeds = [
            TRUSTEE_RECEIPT_SEEDS,
            property_system.key().as_ref(),
            trustee.key().as_ref()
        ],
        bump = trustee_receipt.bump,
    )]
    pub trustee_receipt: Account<'info,TrusteeRecepit>,

#[account(
    mut,
    seeds=[
        TRUSTEEREGISTRYSEEDS,
        property_system.key().as_ref()
    ],
    bump = trustee_registry.bump,
    // constraint = trustee_registry.trustees.contains(&trustee.key()) @ ErrorCode::NotAuthorized
)]
pub trustee_registry : Account<'info,TrusteeRegistry>,

#[account(
    mut,
    associated_token::mint = mint,
    associated_token::authority = trustee_registry, 
    associated_token::token_program = token_program
)]
pub trustee_registry_ata : InterfaceAccount<'info,TokenAccount>,

pub system_program:Program<'info,System>,

}

#[access_control(check_property_system(&ctx.accounts.property_system))]
pub fn trustee_salary_claim(
    ctx:Context<ClaimTrusteeSalary>,
    _property_system_id:u64
)->Result<()>{


    let trustee_registry_pda = &mut ctx.accounts.trustee_registry;

    let trustee_receipt = &mut ctx.accounts.trustee_receipt;

    // let trustee_salary_pda = &mut ctx.accounts.trustee_salary_pda;

    require!(trustee_registry_pda.claim_deadline_ts > trustee_receipt.new_transaction_time,ErrorCode::DeadlineReached);

    if trustee_receipt.new_transaction_time != 0 as i64  {

        let salary = trustee_registry_pda.total_salary_allocated
                                            .checked_div(trustee_registry_pda.total_trustees as u64)
                                            .ok_or(ErrorCode::MathOverflow)?;

    trustee_receipt.new_transaction_time = trustee_registry_pda.claim_deadline_ts;

    let property_sys_key =   ctx.accounts.property_system.key();

    let signer_seeds: &[&[&[u8]]] = &[&[
        TRUSTEEREGISTRYSEEDS,
        property_sys_key.as_ref(),
        &[ctx.accounts.trustee_registry.bump],
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

    }

    else {
        trustee_receipt.new_transaction_time = trustee_registry_pda.claim_deadline_ts;
    }

    Ok(())


}