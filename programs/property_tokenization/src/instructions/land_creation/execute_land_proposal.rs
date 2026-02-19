use anchor_lang::prelude::*;

use crate::state::{Country, LandAccount, LandAccountMetadata, LandPage, LandProposal, PropertySystemAccount, State};

use crate::errors::ErrorCode::{self};

const PROPOSAL_SEEDS: &[u8] = b"proposal";

const STATE_SEEDS : &[u8] = b"state";

const  COUNTRY_SEED : &[u8] = b"country";

use crate::constant::*;

#[derive(Accounts)]

pub struct ExecuteLandProposal<'info>{

    #[account(
        mut,
    )]
    pub property_system_account : Account<'info,PropertySystemAccount>,


    #[account(
        mut,
        seeds = [
            LAND_PAGE_SEEDS,
            &landpage.page.to_le_bytes(),
            property_system_account.key().as_ref()
        ],
        bump = landpage.bump,
        constraint = landpage.property_system == property_system_account.key() @ ErrorCode::PropertySystemInvalid
    )]

    pub landpage : Account<'info,LandPage>,

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
            &land_proposal.land_id.to_le_bytes(),
            state.key().as_ref(),
            country.key().as_ref(),
        ],
        bump = land_proposal.bump,
        constraint = land_proposal.approved @ ErrorCode::ProposalNotApproved,
        constraint = land_proposal.state_pubkey == state.key() @ ErrorCode::InvalidLand,
        close = signer
    )]

    pub land_proposal: Account<'info,LandProposal>,

    #[account(
        mut,
        constraint = state.authorities.contains(&signer.key()) @ ErrorCode::NotAuthorized
    )]

    pub signer: Signer<'info>,

    #[account(
        init,
        payer = signer,
        seeds = [
                LAND_SEED,
                &land_proposal.land_id.to_le_bytes(),
                state.key().as_ref(),
                country.key().as_ref(),
        ],
        bump,
        space = 8 + LandAccount::SIZE
    )]

    pub land_pda : Account<'info,LandAccount>,

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
        space = 8 + LandAccountMetadata::SIZE
    )]
    pub land_metadata : Account<'info,LandAccountMetadata>,

    pub system_program : Program<'info,System>,

}

pub fn execute(
    ctx:Context<ExecuteLandProposal>
)->Result<()>{

    let proposal = &mut ctx.accounts.land_proposal ;

    let land_acc = &mut ctx.accounts.land_pda;

    let metadata = &mut ctx.accounts.land_metadata;

    let land_page  = &mut ctx.accounts.landpage;

    let state = & ctx.accounts.state;

    let country = &mut ctx.accounts.country;

    let current_time = Clock::get()?.unix_timestamp;
    
    require!(!land_page.land.len() < 100 , ErrorCode::PageFull);

    land_page.land.push(land_acc.key());

    land_acc.land_id = proposal.land_id;

    land_acc.property_system = ctx.accounts.property_system_account.key();

    land_acc.page_number = land_page.page;
 
    land_acc.state_id = state.state_id;

    land_acc.state_pubkey = state.key();

    land_acc.country_id  = country.country_id;

    land_acc.country_pubkey = country.key();

    land_acc.issued_at  = current_time;

    land_acc.issued_by = proposal.issued_by;

    land_acc.metadata = metadata.key();

    land_acc.bump = ctx.bumps.land_pda;
   
    metadata.land = land_acc.key();

    metadata.legal_doc_hash = proposal.legal_doc_hash;
    
    metadata.last_updated = current_time;

    metadata.bump = ctx.bumps.land_metadata;

    proposal.executed = true;
    
    Ok(())
}