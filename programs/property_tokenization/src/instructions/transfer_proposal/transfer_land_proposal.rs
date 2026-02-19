use anchor_lang::prelude::*;
use crate::constant::*;
use crate::errors::ErrorCode;
use crate::state::{LandAccount, LandPage, PropertySystemAccount, TransferLandDetail, TreasuryPda, TrusteeRegistry};


#[derive(Accounts)]
#[instruction(proposal_id : u64)]
pub struct TransferLandProposal<'info>{

    #[account(
        init,
        payer = signer,
        seeds =[
            TRANSFERPROPOSAL,
            &proposal_id.to_le_bytes(),
            source_property_system.key().as_ref()
        ],
        bump,
        space = TransferLandDetail::SIZE
    )]

    pub proposal : Account<'info,TransferLandDetail>,

    #[account(
        mut,
        constraint = trustee_registry.trustees.contains(&signer.key()) @ ErrorCode::NotAuthorized
    )]

    pub signer:Signer<'info>,

    #[account(
        seeds =[
                PROPERTY_SYSTEM_SEEDS,
                &source_property_system.property_system_id.to_le_bytes()
        ],
        bump = source_property_system.bump,
    )]
    pub source_property_system:Account<'info,PropertySystemAccount>,

    #[account(
        constraint = source_property_system.trustee_registry == trustee_registry.key() @ ErrorCode::InvalidTrusteeRegsitry
    )]
    pub trustee_registry: Account<'info,TrusteeRegistry>,

    #[account(
        seeds = [
            b"treasury",
            &source_property_system.key().as_ref(),
        ],
        bump = source_treasurypda.bump
    )]
    
    pub source_treasurypda : Account<'info,TreasuryPda>,


    #[account(
        seeds=[ 
            LAND_SEED,
            &land_account.land_id.to_le_bytes(),                
            &land_account.state_pubkey.as_ref(),
            &land_account.country_pubkey.as_ref(),                
        ],bump = land_account.bump,
        constraint = land_account.property_system == source_property_system.key() @ ErrorCode::InvalidLandForSource
    )]
    pub land_account : Account<'info,LandAccount>,


   #[account(
        mut,
        seeds = [
            LAND_PAGE_SEEDS,
            &source_landpage.page.to_le_bytes(),
            &source_property_system.key().as_ref()
        ],
        bump = source_landpage.bump,
        constraint = source_landpage.property_system == source_property_system.key() @ ErrorCode::PropertySystemInvalid
    )]

    pub source_landpage : Account<'info,LandPage>,


     #[account(
        seeds =[
                PROPERTY_SYSTEM_SEEDS,
                &destination_property_system.property_system_id.to_le_bytes()
        ],
        bump = destination_property_system.bump,
    )]
    pub destination_property_system:Account<'info,PropertySystemAccount>,

    #[account(
        seeds = [
            b"treasury",
            &destination_property_system.key().as_ref(),
        ],
        bump = destination_treasurypda.bump
    )]
    
    pub destination_treasurypda : Account<'info,TreasuryPda>,


    pub system_program: Program<'info,System>,

}


pub fn transfer_proposal(ctx:Context<TransferLandProposal>,proposal_id: u64,amount:u64)->Result<()>{

    let source_property_system = &ctx.accounts.source_property_system ;

    let source_land_page = &ctx.accounts.source_landpage;

    let land_account = & ctx.accounts.land_account ;

    let source_treasury_pda = & ctx.accounts.source_treasurypda;

    let proposal = &mut ctx.accounts.proposal;

    let destination_property_system = & ctx.accounts.destination_property_system;

    let destination_treasurypda = &ctx.accounts.destination_treasurypda;

    require!( source_property_system.key() != destination_property_system.key(), ErrorCode::SamePropertySystem);

    require!(source_land_page.land.contains(&land_account.key()), ErrorCode::InvalidLand);

    proposal.proposal_id = proposal_id;

    proposal.land = land_account.key();

    proposal.source_property_system = source_property_system.key();

    proposal.source_treasury = source_treasury_pda.key();

    proposal.destination_property_system = destination_property_system.key();

    proposal.destination_treasury =  destination_treasurypda.key(); 
    
    proposal.amount_to_transfer = amount;

    Ok(())


}