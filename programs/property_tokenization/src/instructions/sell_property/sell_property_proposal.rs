use anchor_lang::prelude::*;
use crate::common::{ PROPERTY_SEED, PROPERTY_SYSTEM_SEEDS, SELLPROPERTY, TREASURYSEEDS, TRUSTEE_RECEIPT_SEEDS, TRUSTEEREGISTRYSEEDS};
use crate::errors::ErrorCode;
use crate::state::{PropertyAccount, PropertySellProposal, PropertySystemAccount, TreasuryPda, TrusteeRecepit, TrusteeRegistry};


#[derive(Accounts)]
#[instruction(proposal_id : u64,property_id:u64,property_system_id:u64,state_pubkey:Pubkey)]
pub struct SellLandProposal<'info>{

    #[account(
        mut,
    )]

    pub trustee:Signer<'info>,

    #[account(
        seeds = [
            TRUSTEE_RECEIPT_SEEDS,
            seller.key().as_ref(),
            trustee.key().as_ref()
        ],
        bump = trustee_receipt.bump,
    )]
    pub trustee_receipt: Account<'info,TrusteeRecepit>,

    #[account(
        init,
        payer = trustee,
        seeds =[
            SELLPROPERTY,
            seller.key().as_ref(),
            &proposal_id.to_le_bytes(),
        ],
        bump,
        space = 8 +PropertySellProposal::SIZE
    )]
    pub proposal : Account<'info,PropertySellProposal>,

    #[account(
        seeds =[
                PROPERTY_SYSTEM_SEEDS,
                &property_system_id.to_le_bytes()
        ],
        bump = seller.bump,
    )]
    pub seller:Account<'info,PropertySystemAccount>,

    // #[account(
    //     seeds = [
    //         TRUSTEEREGISTRYSEEDS,
    //         seller.key().as_ref()
    //     ],
    //     bump = trustee_registry.bump,
    //     // constraint = seller.trustee_registry == trustee_registry.key() @ ErrorCode::InvalidTrusteeRegsitry
    // )]
    // pub trustee_registry: Account<'info,TrusteeRegistry>,

    #[account(
        seeds = [
            TREASURYSEEDS,
            seller.key().as_ref(),
        ],
        bump = seller_treasury.bump
    )]
    
    pub seller_treasury : Account<'info,TreasuryPda>,


    #[account(
        seeds=[ 
            PROPERTY_SEED,
            &property_id.to_le_bytes(),                
            state_pubkey.as_ref(),                
        ],bump = property_account.bump,
        constraint = property_account.property_system == seller.key() @ ErrorCode::InvalidLandForSource,
        constraint = property_account.state_pubkey == state_pubkey @ ErrorCode::InvalidLandForSource,
        constraint = !property_account.is_leased @ ErrorCode::LandCurrentlyLeased,
    )]
    pub property_account : Account<'info,PropertyAccount>,

    pub system_program: Program<'info,System>,

}


pub fn create_sell_proposal(ctx:Context<SellLandProposal>,proposal_id: u64,_property_id:u64,_property_system_id:u64,_state_pubkey:Pubkey,sale_price:u64,)->Result<()>{

    let seller = &ctx.accounts.seller ;

    let property_account = & ctx.accounts.property_account ;

    let seller_treasury = & ctx.accounts.seller_treasury;

    let proposal =&mut ctx.accounts.proposal;

    proposal.initialize(
    proposal_id,
    property_account.key(),
    seller.key(),
    seller_treasury.key(),
    sale_price,
    seller.total_token_supply,
    ctx.bumps.proposal,
    );
    
    Ok(())


}