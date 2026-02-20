use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token_interface::{TokenAccount, TokenInterface}};

use crate::{constant::{HARDCODED_PUBKEY, LAND_PAGE_SEEDS, LAND_SEED, PROPERTY_SYSTEM_SEEDS, ProposalStatus, TRANSFERPROPOSAL}, errors::ErrorCode, state::{LandAccount, LandPage, PropertySystemAccount, ReinvestmentPda, TransferLandDetail, TreasuryPda, TrusteeRegistry}};


#[derive(Accounts)]

pub struct ExecuteTranferProposal<'info>{

    #[account(
        mut,
        constraint = destination_trustee_registry.trustees.contains(&destination_trustee.key()) @ ErrorCode::NotAuthorized,
    )]
    pub destination_trustee :Signer<'info>,

    #[account(
        mut,
        seeds=[
            TRANSFERPROPOSAL,
            source_property_system.key().as_ref(),
            &proposal.proposal_id.to_le_bytes(),
        ],
        bump= proposal.bump,
        constraint = proposal.proposal_status == ProposalStatus::Passed as u8 @ ErrorCode::ProposalNotPassed,
        constraint = proposal.source_property_system == source_property_system.key() @ ErrorCode::InvalidProposal,
        constraint = proposal.destination_property_system == destination_property_system.key() @ ErrorCode::InvalidProposal,

    )]
    pub proposal: Account<'info,TransferLandDetail>,   


    #[account(
        mut,
        seeds=[
            LAND_SEED,
            &land_acc.land_id.to_le_bytes(),
            land_acc.state_pubkey.as_ref(),
            land_acc.country_pubkey.as_ref(),
        ],
        bump = land_acc.bump,
        constraint = land_acc.key() == proposal.land @ ErrorCode::InvalidLand,
        constraint = land_acc.property_system == source_property_system.key() @ ErrorCode::InvalidLandForSource,
    )]
    pub land_acc :Account<'info,LandAccount>,

    #[account(
        seeds=[
            LAND_PAGE_SEEDS,
            &source_land_page.page.to_le_bytes(),
            source_property_system.key().as_ref()
        ],
        bump = source_land_page.bump,
        constraint = source_land_page.land.contains(&land_acc.key()) @ ErrorCode::LandAccountNotFound,
        constraint = source_land_page.property_system == source_property_system.key() @ ErrorCode::LandPageInvalid
    )]
    pub source_land_page:Account<'info,LandPage>,

    #[account(
        seeds=[
            PROPERTY_SYSTEM_SEEDS,
            &source_property_system.property_system_id.to_le_bytes(),
        ],
        bump = source_property_system.bump,
    )]
    pub source_property_system: Account<'info,PropertySystemAccount>,


    #[account(
        constraint = source_property_system.treasury == source_treasury_pda.key() @ ErrorCode::InvalidTreasury
    )]
    pub source_treasury_pda: Account<'info,TreasuryPda>,

     #[account(
        associated_token::mint = HARDCODED_PUBKEY ,
        associated_token::authority = source_treasury_pda ,
    )]
    pub source_property_system_treasury_ata: InterfaceAccount<'info,TokenAccount>,

    ///////////////////////////////////////////////////////////////////////////////////////
    
    #[account(
        seeds=[
            PROPERTY_SYSTEM_SEEDS,
            &destination_property_system.property_system_id.to_le_bytes(),
        ],
        bump = destination_property_system.bump
    )]
    pub destination_property_system : Account<'info,PropertySystemAccount>,

     #[account(
        seeds=[
            LAND_PAGE_SEEDS,
            &destination_land_page.page.to_le_bytes(),
            destination_property_system.key().as_ref()
        ],
        bump = destination_land_page.bump,
        constraint = destination_land_page.land.len() < 100 as usize @ ErrorCode::NotEnoughSpace,
        constraint = destination_land_page.property_system == destination_property_system.key() @ ErrorCode::LandPageInvalid
    )]
    pub destination_land_page:Account<'info,LandPage>,

    #[account(
        constraint = destination_property_system.trustee_registry == destination_trustee_registry.key() @ ErrorCode::InvalidTrusteeRegsitry,
    )]
    pub destination_trustee_registry : Account<'info,TrusteeRegistry>,

    #[account(
        constraint = destination_property_system.treasury == destination_treasury_pda.key() @ ErrorCode::InvalidTreasury
    )]
    pub destination_treasury_pda: Account<'info,TreasuryPda>,

    #[account(
        constraint = destination_treasury_pda.reinvenstement_acc == destination_reinvestment_pda.key() @ ErrorCode::InvalidReinvestAccount
    )]
    pub destination_reinvestment_pda :Account<'info,ReinvestmentPda>,

    #[account(
        associated_token::mint = HARDCODED_PUBKEY ,
        associated_token::authority = destination_reinvestment_pda,
    )]
    pub destination_property_system_reinvestment_ata: InterfaceAccount<'info,TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
    
    pub associated_token_program: Program<'info, AssociatedToken>,
    
    pub system_program: Program<'info, System>,

}

    pub fn execute_tranfer_proposal(ctx:Context<ExecuteTranferProposal>)->Result<()>{

        let proposal = &mut ctx.accounts.proposal;

        let source_treasury_ata = &mut ctx.accounts.source_property_system_treasury_ata;

        let destination_reinvestment_ata = &mut ctx.accounts.destination_property_system_reinvestment_ata;

        let source_page = &mut ctx.accounts.source_land_page;

        let destination_page = &mut ctx.accounts.destination_land_page;

        let land_account = &mut ctx.accounts.land_acc;

        let current_time = Clock::get()?.unix_timestamp;

        let threeday:i64 = 60 * 60 * 24 * 3;

        let transfer_window_close = proposal.transfer_window
                                                    .checked_add(threeday)
                                                    .ok_or(ErrorCode::MathOverflow)?;

        require!(proposal.transfer_window <= current_time && current_time<= transfer_window_close,ErrorCode::TransferWindowClose);

        require!(destination_reinvestment_ata.amount >= proposal.amount_to_transfer , ErrorCode::InsufficentBalance);

        ///hold 
        

        Ok(())



    }


