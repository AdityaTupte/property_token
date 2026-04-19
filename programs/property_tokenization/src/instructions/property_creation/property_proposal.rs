use anchor_lang::prelude::*;

use crate::{common::{ COUNTRY_SEED, PROPERTY_PROPOSAL_SEEDS, PROPERTY_SYSTEM_SEEDS, STATE_AUTHORITY, STATE_SEEDS}, errors::ErrorCode, state::{ Country, PropertyProposal, PropertySystemAccount, State, StateAuthority}};



#[derive(Accounts)]
#[instruction(country_key:Pubkey,state_name:[u8;32],property_id:u64,property_system_id:u64)]
pub struct CreatePropertyProposal<'info>{

        #[account(
            mut,
            seeds = [ 
                PROPERTY_SYSTEM_SEEDS,
                &property_system_id.to_le_bytes(),
            ],
            bump= property_system.bump,
            constraint = property_system.ready_for_listing @ ErrorCode::PropertySystemReadyForListing
        )]
        pub property_system : Account<'info,PropertySystemAccount>,
    #[account(
        seeds=[
            STATE_SEEDS,
            state_name.as_ref(),
            country_key.as_ref(),
        ],
        bump = state.bump,
        constraint =  country_key == state.country_pubkey @ ErrorCode::InvalidCountry
    )]
    pub state: Account<'info,State>,

    #[account(
        mut,
    )]
    pub signer : Signer<'info>,

    #[account(
        seeds=[
            STATE_AUTHORITY,
            country_key.as_ref(),
            signer.key().as_ref()
        ],
        bump= state_authority_receipt.bump,
    )]
    pub state_authority_receipt : Account<'info,StateAuthority>,


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
    _country_key:Pubkey,
    _state_name:[u8;32],
    property_id : u64,
    _property_system_id:u64,
    legal_doc_hash: [u8; 32],
)->Result<()>{

        let proposal = &mut ctx.accounts.proposal;

        let state = & ctx.accounts.state;

        proposal.property_system_pubkey = ctx.accounts.property_system.key();

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

