use anchor_lang::prelude::*;

use crate::state::{Country, PropertyAccount, PropertyAccountMetadata, PropertyPage, PropertyProposal, PropertySystemAccount, State};

use crate::errors::ErrorCode::{self};

const PROPOSAL_SEEDS: &[u8] = b"proposal";

const STATE_SEEDS : &[u8] = b"state";

const  COUNTRY_SEED : &[u8] = b"country";

use crate::constant::*;

#[derive(Accounts)]

pub struct ExecutePropertyProposal<'info>{

    #[account(
        mut,
    )]
    pub property_system_account : Account<'info,PropertySystemAccount>,


    #[account(
        mut,
        seeds = [
            PROPERTY_PAGE_SEEDS,
            &property_page.page.to_le_bytes(),
            property_system_account.key().as_ref()
        ],
        bump = property_page.bump,
        constraint = property_page.property_system == property_system_account.key() @ ErrorCode::PropertySystemInvalid
    )]

    pub property_page : Account<'info,PropertyPage>,

    #[account(
        seeds=[
            COUNTRY_SEED,
            &country.country_id.to_le_bytes()
        ],
        bump = country.bump
    )]
    pub country : Account<'info,Country>,

    #[account(
        seeds = [
            STATE_SEEDS,
            &state.state_id.to_le_bytes(),
            country.key().as_ref(),
        ],
        bump = state.bump,
    )]
    pub state: Account<'info,State>,

    #[account(
        mut,
        seeds=[
            PROPOSAL_SEEDS,
            &property_proposal.property_id.to_le_bytes(),
            state.key().as_ref(),
            country.key().as_ref(),
        ],
        bump = property_proposal.bump,
        constraint = property_proposal.approved @ ErrorCode::ProposalNotApproved,
        constraint = property_proposal.state_pubkey == state.key() @ ErrorCode::InvalidProperty,
        close = signer
    )]

    pub property_proposal: Account<'info,PropertyProposal>,

    #[account(
        mut,
        constraint = state.authorities.contains(&signer.key()) @ ErrorCode::NotAuthorized
    )]

    pub signer: Signer<'info>,

    #[account(
        init,
        payer = signer,
        seeds = [
                PROPERTY_SEED,
                &property_proposal.property_id.to_le_bytes(),
                state.key().as_ref(),
                country.key().as_ref(),
        ],
        bump,
        space = 8 + PropertyAccount::SIZE
    )]

    pub land_pda : Account<'info,PropertyAccount>,

    #[account(
        init,
        payer = signer,
        seeds = [
            LAND_METADATA_SEEDS,
            land_pda.key().as_ref(),
            state.key().as_ref(),
            country.key().as_ref()
        ],
        bump,
        space = 8 + PropertyAccountMetadata::SIZE
    )]
    pub land_metadata : Account<'info,PropertyAccountMetadata>,

    pub system_program : Program<'info,System>,

}

pub fn execute(
    ctx:Context<ExecutePropertyProposal>
)->Result<()>{

    let proposal = &mut ctx.accounts.property_proposal ;

    let property_acc = &mut ctx.accounts.land_pda;

    let metadata = &mut ctx.accounts.land_metadata;

    let land_page  = &mut ctx.accounts.property_page;

    let state = & ctx.accounts.state;

    let country = &mut ctx.accounts.country;

    let current_time = Clock::get()?.unix_timestamp;
    
    require!(!land_page.land.len() < 100 , ErrorCode::PageFull);

    land_page.land.push(property_acc.key());

    property_acc.property_id = proposal.property_id;

    property_acc.property_system = ctx.accounts.property_system_account.key();

    property_acc.page_number = land_page.page;
 
    property_acc.state_id = state.state_id;

    property_acc.state_pubkey = state.key();

    property_acc.country_id  = country.country_id;

    property_acc.country_pubkey = country.key();

    property_acc.issued_at  = current_time;

    property_acc.issued_by = proposal.issued_by;

    property_acc.metadata = metadata.key();

    property_acc.bump = ctx.bumps.land_pda;
   
    metadata.land = property_acc.key();

    metadata.legal_doc_hash = proposal.legal_doc_hash;
    
    metadata.last_updated = current_time;

    metadata.bump = ctx.bumps.land_metadata;

    proposal.executed = true;
    
    Ok(())
}