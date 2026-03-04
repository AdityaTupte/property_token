use anchor_lang::prelude::*;
use anchor_spl::token_interface::{TokenAccount,Mint,TransferChecked,transfer_checked, TokenInterface};
use anchor_spl::associated_token::*;

use crate::{constant::{PROPERTY_SYSTEM_SEEDS, SAFETYPROPOSAL}, errors::ErrorCode, state::{PropertySystemAccount, SafetyPda, SafetyProposal, TreasuryPda, TrusteeRegistry}};
use crate::constant::*;

#[derive(Accounts)]
pub struct ExecuteSafety<'info>{

    #[account(
        mut,
        constraint = trustee_registry.trustees.contains(&trustee.key()) @ ErrorCode::NotAuthorized
    )]
    pub trustee:Signer<'info>,

    #[account(
        seeds=[
            SAFETYPROPOSAL,
            property_system.key().as_ref(),
            &proposal.proposal_id.to_le_bytes(),
        ],
        bump = proposal.bump,
        constraint = proposal.status == ProposalStatus::Passed @ ErrorCode::ProposalNotPassed,
        constraint = proposal.property_system == property_system.key() @ ErrorCode::InvalidProposal,
    )]
    pub proposal: Account<'info,SafetyProposal>,

    #[account(
        seeds=[
            PROPERTY_SYSTEM_SEEDS,
            &property_system.property_system_id.to_le_bytes(),
        ],
        bump = property_system.bump
    )]
    pub property_system: Account<'info,PropertySystemAccount>,

    #[account(
         constraint =  property_system.trustee_registry == trustee_registry.key() @ ErrorCode::InvalidTrusteeRegsitry
    )]
    pub trustee_registry : Account<'info,TrusteeRegistry>,

    #[account(
        constraint = property_system.treasury == treasury.key() @ ErrorCode::InvalidTreasury,
    )]
    pub treasury : Account<'info,TreasuryPda>,

    #[account(
        constraint = treasury.safety_acc == safety_treasury.key() @ ErrorCode::InvalidSafety,
        seeds=[
            b"safety",
            property_system.key().as_ref()
        ],
        bump = safety_treasury.bump,
    )]
    pub safety_treasury: Account<'info,SafetyPda>,

    #[account(
        associated_token::mint = mint,
        associated_token::authority = safety_treasury,
    )]
    pub safety_ata:InterfaceAccount<'info,TokenAccount>,

    #[account(
        constraint = proposal.recepient_wallet == recepient_wallet.key(),
    )]
    pub recepient_wallet : SystemAccount<'info>,

    #[account(
        init_if_needed,
        payer = trustee,
        associated_token::mint = mint,
        associated_token::authority = recepient_wallet,
    )]
    pub recepient_ata: InterfaceAccount<'info, TokenAccount>,

    pub mint: InterfaceAccount<'info, Mint>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program : Interface<'info,TokenInterface>,
    pub system_program : Program<'info,System>,
} 

pub fn execute_safety_proposal(ctx:Context<ExecuteSafety>)->Result<()>{

    let proposal = &mut ctx.accounts.proposal;

    let current_time = Clock::get()?.unix_timestamp;

    let property_system =  ctx.accounts.property_system.key();

    require!(current_time <= proposal.deadline, ErrorCode::CantTramnsfer );

    let cpi_accounts = TransferChecked{
        from: ctx.accounts.safety_ata.to_account_info(),
        to: ctx.accounts.recepient_ata.to_account_info(),
        authority:ctx.accounts.safety_treasury.to_account_info(),
        mint : ctx.accounts.mint.to_account_info(),
    };

    let cpi_token_program = ctx.accounts.token_program.to_account_info();

    let signer_seeds :&[&[&[u8]]] = &[&[ 
            SAFETYPROPOSAL,
            property_system.as_ref(),
            &[ctx.accounts.safety_treasury.bump]
        ]];
    
    let cpi_context = CpiContext::new_with_signer(
            cpi_token_program,
            cpi_accounts, 
            signer_seeds);

    transfer_checked(cpi_context, ctx.accounts.proposal.amount_required, ctx.accounts.mint.decimals)?;

    Ok(())


}