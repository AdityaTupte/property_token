use std::collections::BTreeSet;

use anchor_lang::prelude::*;

use crate::common::{COUNTRY_SEED, STATE_PROPOSAL_SEEDS};
use crate::state::{Country, StateProposalPda};
use crate::errors::ErrorCode;

#[derive(Accounts)]
#[instruction(state_id : u16)]
pub struct StateProposal<'info>{

    #[account(
        seeds = [
            COUNTRY_SEED,
            &country.country_id.to_le_bytes()
            ],
        bump = country_pda.bump
    )]
    pub country : Account<'info,Country>,

    #[account(
        mut,
        constraint = country.authority.contains(&signer.key()) @ ErrorCode::NotAuthorized
    )]

    pub signer : Signer<'info>,    

    #[account(
        init,
        payer = signer,
        seeds = [
            STATE_PROPOSAL_SEEDS,
            &state_id.to_le_bytes(),
            country.key().as_ref(),
        ],
        bump,
        space = 8 + StateProposalPda::SIZE,
    )]

    pub state_proposal: Account<'info,StateProposalPda>,

    pub system_program : Program<'info,System>,

}

pub fn create_state_proposal(
    ctx:Context<StateProposal>,
    state_id : u16,
    state_name: String,
    state_authorities : Vec<Pubkey>,
    state_authority_threshold: u8,
)->Result<()>{

    require!(state_name.len() > 0 && state_name.len() <= 32,ErrorCode::StateNameInvalid);
    require!( state_authorities.len() == 10 ,ErrorCode::StateAuthorityInvalid );
    require!(state_authority_threshold > 0 && state_authority_threshold <=10 , ErrorCode::StateThresholdInvalid);

    let unique: BTreeSet<Pubkey> = state_authorities.iter().cloned().collect();
    
    require!(unique.len()  == 10 , ErrorCode::DuplicateAuthority);

    let proposal = &mut ctx.accounts.state_proposal;

    let country = & ctx.accounts.country;

    proposal.state_id = state_id;

    proposal.state_name = state_name.to_uppercase();

    proposal.state_authorities = state_authorities;

    proposal.state_authority_threshold = state_authority_threshold;

    proposal.country_id = country.country_id;

    proposal.country_pubkey = country.key();

    proposal.bump = ctx.bumps.state_proposal;

    Ok(())
}