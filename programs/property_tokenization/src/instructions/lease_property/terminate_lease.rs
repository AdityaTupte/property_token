use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken,  token_interface::{Mint, TokenAccount,transfer_checked, TokenInterface, TransferChecked}};

use crate::{common::{HARDCODED_PUBKEY, LEASE_PROPERTY, LeaseStatus, PROPERTY_SEED, REINVESTMENTPDA}, errors::ErrorCode, instructions::FinalizeLease, state::{LeaseProperty, PropertyAccount, ReinvestmentPda }};

#[derive(Accounts)]
pub struct TerminateLease<'info>{

    pub neutral : Signer<'info>,

    #[account(
        seeds=[
            LEASE_PROPERTY,
            &lease.lease_id.to_le_bytes(),
            property.key().as_ref()
        ],
        bump = lease.bump,
        constraint = lease.status == LeaseStatus::Active @ ErrorCode::LeaseNotActivated,
    )]

     pub lease: Account<'info,LeaseProperty>,

    #[account(
        seeds = [
                    PROPERTY_SEED,
                    &property.property_id.to_le_bytes(),
                    property.state_pubkey.as_ref(),
            ],
            bump = property.bump,
            constraint = property.is_leased @ ErrorCode::LeaseNotActivated
           
    )]
    pub property:Account<'info,PropertyAccount>,


     #[account(
        seeds = [
            REINVESTMENTPDA,
            property.property_system.key().as_ref()],
        bump= reinvestment_pda.bump ,
    )]
    pub reinvestment_pda :Account<'info,ReinvestmentPda>,

    #[account(
        associated_token::mint = mint,
        associated_token::authority = reinvestment_pda,
        associated_token::token_program = token_program,
    )]
    pub reinvestment_ata : InterfaceAccount<'info,TokenAccount>,

    pub system_program : Program<'info,System>,
    
    #[account(
        address = HARDCODED_PUBKEY
    )]
    pub mint : InterfaceAccount<'info,Mint>,

    pub token_program : Interface<'info,TokenInterface>,

    pub associated_token_program : Program<'info,AssociatedToken>,



}

pub fn terminate_lease(
    ctx:Context<FinalizeLease>,
    send_security_deposit_to_lessee:u64,
)->Result<()>{


     let now = Clock::get()?.unix_timestamp;
    let decimals = ctx.accounts.mint.decimals;
    let lease = &mut ctx.accounts.lease;

    require!(now < lease.lease_end_time , ErrorCode::LeaseEndTimeReached );

    let property_acc_key =  ctx.accounts.property.key();
    
    
    let cpi_account = TransferChecked{
        from:ctx.accounts.lease_ata.to_account_info(),
        mint:ctx.accounts.mint.to_account_info(),
        to:ctx.accounts.lessee_ata.to_account_info(),
        authority:lease.to_account_info(),
    };

    let signer_seeds: &[&[&[u8]]]  = &[&[
        LEASE_PROPERTY,
        &[lease.lease_id as u8],
        property_acc_key.as_ref(),
        &[lease.bump]
        ]];


    let ctx1 = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        cpi_account,
         signer_seeds
        );

    transfer_checked(ctx1, send_security_deposit_to_lessee, decimals)?;


    let cpi_accounts2 = TransferChecked{
        from:ctx.accounts.lease_ata.to_account_info(),
        mint:ctx.accounts.mint.to_account_info(),
        to: ctx.accounts.reinvestment_ata.to_account_info(),
        authority:ctx.accounts.reinvestment_pda.to_account_info(),
    };

    let ctx2 = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        cpi_accounts2,
         signer_seeds
        );

    transfer_checked(ctx2, lease.security_deposit-send_security_deposit_to_lessee, decimals)?;


    lease.status = LeaseStatus::Terminated;     

    
    Ok(())

}