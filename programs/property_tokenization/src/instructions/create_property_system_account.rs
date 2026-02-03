use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token_interface::{Mint, TokenAccount, TokenInterface}};
use crate::state::{ArbitratorRegistry, PropertySystemCounter, PropertySystemAccount, TreasuryPda, TrusteeRegistry};

#[derive(Accounts)]
#[instruction(decimal:u8)]
pub struct PropertySystemAcc<'info>{


    #[account(mut)]
    pub creator: Signer<'info>,

    pub property_system_count : Account<'info,PropertySystemCounter>,

    #[account(
        init,
        payer = creator,
        seeds = [ 
            b"property-system-account".as_ref(),
            &(property_system_count.total_property_system + 1).to_le_bytes()],
        bump,
        space = 8 + PropertySystemAccount::SIZE
    )]

    pub property_system_acc : Account<'info,PropertySystemAccount>,

    #[account(
        init,
        payer = creator,
        seeds = [
            b"treasury",
            property_system_acc.key().as_ref()],
        bump,
        space = 8 + TreasuryPda::SIZE,
    )]

    pub treasury_acc : Account<'info,TreasuryPda>,

    #[account(
        init,
        payer = creator,
        seeds = [
            b"trustee_registry",
            property_system_acc.key().as_ref()],
        bump,
        space = 8 + TrusteeRegistry::SIZE,
    )]
    pub trustee_registry : Account<'info,TrusteeRegistry>,
    
    
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
        mint::decimals = decimal,
        mint::authority = property_system_acc.key(),
        mint::freeze_authority = property_system_acc.key(),
    )]
    pub governance_mint: InterfaceAccount<'info, Mint>,

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