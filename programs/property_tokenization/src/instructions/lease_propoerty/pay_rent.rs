use anchor_lang::prelude::*;
use anchor_spl::{ associated_token::AssociatedToken, token_interface::{Mint, TokenAccount, TokenInterface, TransferChecked, transfer_checked}};

use crate::{common::{HARDCODED_PUBKEY, LEASE_PROPERTY, LeaseStatus, TREASURYSEEDS}, errors::ErrorCode,  state::{LeaseProperty, TreasuryPda}};


#[derive(Accounts)]

pub struct PayRent<'info>{

    #[account(
        constraint = signer.key() == lease.lessee @ ErrorCode::NotAuthorized
    )]
    pub signer: Signer<'info>,
    
    #[account(
        associated_token::mint = mint,
        associated_token::authority = signer,
    )]
    pub signer_ata : InterfaceAccount<'info,TokenAccount>,

    #[account(
        mut,
        seeds=[
                TREASURYSEEDS,
                lease.property_system.as_ref(),
        ],
        bump = treasury.bump
    )]
    pub treasury:Account<'info,TreasuryPda>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = treasury,
    )]
    pub treasury_ata : InterfaceAccount<'info,TokenAccount>,

    #[account(
        seeds=[
            LEASE_PROPERTY,
            &lease.lease_id.to_le_bytes(),
            lease.property.as_ref()
        ],
        bump= lease.bump,
        constraint = lease.status == LeaseStatus::Active @ ErrorCode::LeaseNotActivated,
    )]
    pub lease : Account<'info,LeaseProperty>,

    #[account(
        address = HARDCODED_PUBKEY,
    )]
    pub mint : InterfaceAccount<'info,Mint>,

    pub associated_token_program: Program<'info, AssociatedToken>,

    pub token_program : Interface<'info,TokenInterface>

}

 pub fn pay_rent(
    ctx:Context<PayRent>
 )->Result<()>{


    let lease = &mut ctx.accounts.lease;

    let now = Clock::get()?.unix_timestamp;


    require!(lease.next_payment < lease.lease_end_time ,ErrorCode::LeaseEndTimeReached);

    require!(now >= lease.next_payment , ErrorCode::BillNotGenrated);

    let grace_deadline = lease.next_payment
    .checked_add(86400)
    .ok_or(ErrorCode::MathOverflow)?;

    let late_time = now.saturating_sub(grace_deadline);

 
    let days_late = (late_time as u64 + 86400 - 1) / 86400;
   
    let late_fee = lease.late_payment_fee_per_day
    .checked_mul(days_late)
    .ok_or(ErrorCode::MathOverflow)?;

    let total_amount_to_pay = lease.rent_amount.checked_add(late_fee).ok_or(ErrorCode::MathOverflow)?;

    require!(total_amount_to_pay <= ctx.accounts.signer_ata.amount,ErrorCode::InsufficentBalance);

    lease.next_payment = lease.next_payment.checked_add(lease.periodic_pay).ok_or(ErrorCode::MathOverflow)?;

    lease.last_payement = now;

    let cpi_accounts = TransferChecked{
        from: ctx.accounts.signer_ata.to_account_info(),
        mint : ctx.accounts.mint.to_account_info(),
        to:ctx.accounts.treasury_ata.to_account_info(),
        authority : ctx.accounts.signer.to_account_info()
    };

    
    let ctx1 = CpiContext::new(
        ctx.accounts.token_program.to_account_info(), 
        cpi_accounts);

    transfer_checked(ctx1,  total_amount_to_pay,ctx.accounts.mint.decimals)?;
    

Ok(())

 }