use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token_interface::{Mint,MintToChecked,mint_to_checked, TokenAccount, TokenInterface}};
use crate::state::{ArbitratorRegistry, DividendPda, PropertySystemAccount, PropertySystemCounter, ReinvestmentPda, SafetyPda, Threshold, TreasuryPda, TrusteeRegistry,};
use crate::events::*;
use crate::errors::ErrorCode;
use crate::constant::*;



#[derive(Accounts)]
#[instruction(decimal:u8 ,system_id : u64 )]
pub struct CreatePropertySystem<'info>{

    #[account(mut)]
    pub creator: Signer<'info>,

    #[account(
        init,
        payer = creator,
        seeds = [ 
            PROPERTY_SYSTEM_SEEDS,
            &system_id.to_le_bytes(),
        ],
        bump,
        space = 8 + PropertySystemAccount::SIZE
    )]

    pub property_system_acc : Account<'info,PropertySystemAccount>,

    pub system_program: Program<'info, System>,
}