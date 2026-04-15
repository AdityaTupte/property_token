

use anchor_lang::prelude::*;

use crate::common::{COUNTRY_AUTHORITY, COUNTRY_SEED, STATE_PROPOSAL_SEEDS};
use crate::state::{Country, CountryAuthority, StateProposalPda};
use crate::errors::ErrorCode;

#[derive(Accounts)]
#[instruction(
    state_name:[u8;32],
    country_name:[u8;32])]
pub struct StateProposal<'info>{

    #[account(
        seeds = [
            COUNTRY_SEED,
            country_name.as_ref()
            ],
        bump = country.bump
    )]
    pub country : Account<'info,Country>,

    #[account(
        mut,
    )]

    pub signer : Signer<'info>,  

    #[account(
        seeds=[
            COUNTRY_AUTHORITY,
            country.key().as_ref(),
            signer.key().as_ref(),
        ],
        bump=country_authority.bump
    )] 
    pub country_authority : Account<'info,CountryAuthority>,

    #[account(
        init,
        payer = signer,
        seeds = [
            STATE_PROPOSAL_SEEDS,
            state_name.as_ref(),
            country.key().as_ref(),
        ],
        bump,
        space = 8 + StateProposalPda::SIZE
    )]

    pub state_proposal: Account<'info,StateProposalPda>,

    pub system_program : Program<'info,System>,

}

pub fn create_state_proposal(
    ctx:Context<StateProposal>,
    state_name: [u8;32],
    _country_name: [u8;32],
    state_id : u16,
    state_total_authorities :u8,
    state_authority_threshold: u8,
    
)->Result<()>{

    require!(state_name.len() > 0 && state_name.len() <= 32,ErrorCode::StateNameInvalid);
   
    require!(state_authority_threshold > 0 && state_authority_threshold <=10 , ErrorCode::StateThresholdInvalid);
    require!(
        state_name.iter().any(|&c| c != 0),
        ErrorCode::CountryNameInvalid
    );
    require!(
        state_name.iter().all(|&c| c == 0 || (c >= b'A' && c <= b'Z')),
        ErrorCode::NotInUppercase
    );

    // assert_unique_owners(&state_authorities)?;
    
    //require!(state_authorities.len()  == 10 , ErrorCode::DuplicateAuthority);

    let proposal = &mut ctx.accounts.state_proposal;

    let country = & ctx.accounts.country;

    proposal.state_id = state_id;

    proposal.state_name = state_name;

    // proposal.state_authorities = state_authorities;

    proposal.state_total_authorities = state_total_authorities;

    proposal.state_authority_threshold = state_authority_threshold;

    proposal.country_id = country.country_id;

    proposal.country_pubkey = country.key();

    proposal.bump = ctx.bumps.state_proposal;

    Ok(())
}