use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token_interface::{Mint,transfer_checked, TokenAccount, TokenInterface, TransferChecked}};

use crate::{common::{ LEASE_PROPERTY, LEASE_PROPERTY_PROPOSAL, LeaseStatus, PROPERTY_SYSTEM_SEEDS, ProposalStatus, TREASURYSEEDS}, errors::ErrorCode, state::{LeaseProperty, LeaseProposal, PropertyAccount, PropertySystemAccount, TreasuryPda}};



#[derive(Accounts)]
#[instruction(property_system_id:u64,lease_id:u64)]
pub struct LesseeAcceptance<'info>{

    #[account(
        mut,
        constraint = proposal.lessee == signer.key() @ ErrorCode::UnAuthorized 
    )]
    pub signer : Signer<'info>,


    #[account(
        init,
        payer = signer,
        seeds=[
            LEASE_PROPERTY,
            property_system.key().as_ref(),
            property.key().as_ref(),
            &lease_id.to_le_bytes(),
        ],
        bump,
        space = 8 + LeaseProperty::SIZE
    )]
    pub lease : Box<Account<'info,LeaseProperty>>,

   #[account(
        mut,
        constraint = proposal.neutral == neutral.key() @ ErrorCode::InvalidAuthorityMapping 
    )]
    pub neutral : SystemAccount<'info>,

    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = mint,
        associated_token::authority = lease,
        associated_token::token_program = token_program,
    )]
    pub lease_ata : InterfaceAccount<'info,TokenAccount>,


    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = signer,
        associated_token::token_program = token_program,
    )]
    pub signer_ata : InterfaceAccount<'info,TokenAccount>,

    #[account(
        // seeds = [
        //             PROPERTY_SEED,
        //             &property.property_id.to_le_bytes(),
        //             property.state_pubkey.as_ref(),
        //     ],
        //     bump = property.bump,
            constraint = !property.is_leased @ ErrorCode::LeaseActivated
           
    )]
    pub property : Box<Account<'info,PropertyAccount>>,

    #[account(
        seeds=[
            PROPERTY_SYSTEM_SEEDS,
            &property_system_id.to_le_bytes(),
        ],
        bump = property_system.bump
    )]
    pub property_system : Box<Account<'info,PropertySystemAccount>>,

    #[account(
        seeds = [
            TREASURYSEEDS,
            property_system.key().as_ref()],
        bump= treasury_pda.bump ,
    )]
    pub treasury_pda : Box<Account<'info,TreasuryPda>>,

    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = mint,
        associated_token::authority = treasury_pda,
        associated_token::token_program = token_program,
    )]
    pub treasury_ata : InterfaceAccount<'info,TokenAccount>,

    #[account(
        mut,
        seeds=[
            LEASE_PROPERTY_PROPOSAL,
            property_system.key().as_ref(),
            property.key().as_ref(),
            &lease_id.to_le_bytes(),
        ],
        bump= proposal.bump ,
         constraint = proposal.property == property.key() @ ErrorCode::InvalidProposal,
    )]
    pub proposal : Box<Account<'info,LeaseProposal>>,

    pub system_program : Program<'info,System>,
    
    #[account(
        // address = HARDCODED_PUBKEY
    )]
    pub mint : InterfaceAccount<'info,Mint>,

    pub token_program : Interface<'info,TokenInterface>,

    pub associated_token_program : Program<'info,AssociatedToken>,


}


pub fn lessee_acceptance(
    ctx:Context<LesseeAcceptance>,
    _property_system_id:u64,_lease_id:u64
)->Result<()>{


    let now  = Clock::get()?.unix_timestamp;

    let proposal = &mut ctx.accounts.proposal;

    require!(proposal.lessee == ctx.accounts.signer.key(),ErrorCode::UnAuthorized);

    let lease = &mut ctx.accounts.lease;

    require!(now <= proposal.lessee_acceptance_deadline , ErrorCode::DeadlineReached );

    lease.lessee = proposal.lessee;

    lease.lease_start_time = now;

    lease.lease_end_time = proposal.lease_end_time;

    lease.property = ctx.accounts.property.key();

    lease.property_system = ctx.accounts.property_system.key();

    lease.agreemenbt_hash = proposal.agreemenbt_hash;

    lease.rent_amount = proposal.rent_amount;

    lease.security_deposit = proposal.security_deposit;

    lease.neutral = proposal.neutral;

    lease.status = LeaseStatus::Active;

    lease.lease_id = proposal.lease_id;

    lease.last_payement = now;

    lease.status = LeaseStatus::Active;

    proposal.status = ProposalStatus::Executed;

    lease.late_payment_fee_per_day = proposal.late_payment_fee_per_day  ;


    let periodic_payment_in_sec = proposal.periodic_pay
                                        .checked_mul(24*60*60)
                                        .ok_or(ErrorCode::MathOverflow)?;


    lease.periodic_pay = periodic_payment_in_sec;

    lease.next_payment = periodic_payment_in_sec
                                .checked_add(now)
                                .ok_or(ErrorCode::MathOverflow)?;


    lease.bump = ctx.bumps.lease;

    let decimals = ctx.accounts.mint.decimals;





    let cpi_accounts  = TransferChecked{
        from: ctx.accounts.signer_ata.to_account_info(),
        mint : ctx.accounts.mint.to_account_info(),
        to : ctx.accounts.treasury_ata.to_account_info(),
        authority:ctx.accounts.signer.to_account_info(),
    };

    let ctx1 = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        cpi_accounts);

        

    transfer_checked(ctx1, lease.rent_amount,decimals )?;

    lease.last_payement = now;


    let cpi_accounts2  = TransferChecked{
        from: ctx.accounts.signer_ata.to_account_info(),
        mint : ctx.accounts.mint.to_account_info(),
        to : ctx.accounts.lease_ata.to_account_info(),
        authority:ctx.accounts.signer.to_account_info(),
    };

    let ctx2 = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        cpi_accounts2);

    transfer_checked(ctx2, lease.security_deposit,decimals )?;
  


    Ok(())


}
