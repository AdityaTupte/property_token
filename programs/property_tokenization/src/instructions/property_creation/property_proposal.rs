use anchor_lang::prelude::*;

use crate::{common::{ PROPERTY_PROPOSAL_SEEDS, STATE_SEEDS}, errors::ErrorCode, state::{ PropertyProposal, State}};

const PROPOSAL_SEEDS: &[u8] = b"proposal";

#[derive(Accounts)]
#[instruction(property_id:u64)]
pub struct CreatePropertyProposal<'info>{

   

    #[account(
        seeds=[
            STATE_SEEDS,
            &state.state_id.to_le_bytes(),
            state.country_pubkey.as_ref(),
        ],
        bump = state.bump,
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
            PROPERTY_PROPOSAL_SEEDS,
            &property_id.to_le_bytes(),
            state.key().as_ref(),
        ],
        bump,
        space =  8 + PropertyProposal::SIZE
    )]
    pub proposal: Account<'info,PropertyProposal>,

    pub system_program: Program<'info,System>,
}

pub fn create_proposal(
    ctx:Context<CreatePropertyProposal>,
    property_id : u64,
    legal_doc_hash: [u8; 32],
)->Result<()>{

        let proposal = &mut ctx.accounts.proposal;

        let state = & ctx.accounts.state;

        proposal.property_id = property_id;

        proposal.state_id = state.state_id;

        proposal.state_pubkey = state.key();

        proposal.country_id = state.country_id;

        proposal.country_pubkey = state.country_pubkey;

        proposal.legal_doc_hash = legal_doc_hash;

        proposal.issued_by = ctx.accounts.signer.key();

        proposal.bump = ctx.bumps.proposal;

    Ok(())
}

