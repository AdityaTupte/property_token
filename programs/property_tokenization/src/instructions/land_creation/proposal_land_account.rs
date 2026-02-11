use anchor_lang::prelude::*;

use crate::{errors::ErrorCode, state::{Country, LandProposal, State}};

const STATE_SEEDS : &[u8] = b"state";

const PROPOSAL_SEEDS: &[u8] = b"proposal";

#[derive(Accounts)]
#[instruction(land_id:u64)]
pub struct CreateLandProposal<'info>{

    #[account()]
    pub country : Account<'info,Country>,

    #[account(
        seeds=[
            STATE_SEEDS,
            &state.state_id.to_le_bytes(),
            country.key().as_ref(),
        ],
        bump = state.bump,
        constraint = state.country_id == country.country_id @ ErrorCode::InvalidCountry,
        constraint = state.country_pubkey == country.key() @ ErrorCode::InvalidCountry
    )]
    pub state: Account<'info,State>,

    #[account(
        mut,
        constraint = state.authorities.contains(&signer.key()) @ ErrorCode::NotAuthorized
    )]
    pub signer : Signer<'info>,

    #[account(
        init,
        payer = signer,
        seeds = [
            PROPOSAL_SEEDS,
            &land_id.to_le_bytes(),
            state.key().as_ref(),
            country.key().as_ref(),
        ],
        bump,
        space =  8 + LandProposal::SIZE
    )]
    pub proposal: Account<'info,LandProposal>,

    pub system_program: Program<'info,System>,
}

pub fn create_proposal(
    ctx:Context<CreateLandProposal>,
    land_id : u64,
    legal_doc_hash: [u8; 32],
)->Result<()>{

        let proposal = &mut ctx.accounts.proposal;

        let state = &mut ctx.accounts.state;

        let country = &mut ctx.accounts.country;

        proposal.land_id = land_id;

        proposal.state_id = state.state_id;

        proposal.state_pubkey = state.key();

        proposal.country_id = country.country_id;

        proposal.country_pubkey = country.key();

        proposal.legal_doc_hash = legal_doc_hash;

        proposal.issused_by = ctx.accounts.signer.key();

        proposal.bump = ctx.bumps.proposal;

    Ok(())
}

