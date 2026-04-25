use anchor_lang::prelude::*;

use crate::{common::{ COUNTRY_AUTHORITY, COUNTRY_SEED, STATE_AUTHORITY, STATE_PROPOSAL_SEEDS, STATE_SEEDS}, errors::ErrorCode, state::{ Country, CountryAuthority, State, StateAuthority, StateProposalPda}};

#[derive(Accounts)]
#[instruction(country_name: [u8;32],state_name: [u8;32])]
pub struct AddStateAuthority<'info>{


    #[account(mut)]
    pub signer :Signer<'info>,

    pub state_authority : SystemAccount<'info>,

    #[account(
        seeds = [
            COUNTRY_SEED,
            country_name.as_ref()
            ],
        bump = country.bump
    )]
    pub country : Account<'info,Country>,


    #[account(
        seeds=[
            COUNTRY_AUTHORITY,
            country.key().as_ref(),
            signer.key().as_ref()
        ],
        bump = country_authority.bump,
    )]
    pub country_authority : Account<'info,CountryAuthority>,

    #[account{
        mut,
        seeds=[
            STATE_PROPOSAL_SEEDS,
            state_name.as_ref(),
            country.key().as_ref()
        ],
        bump = proposal.bump,
        constraint = proposal.approved @ ErrorCode::ProposalNotApproved,
        
    }]
    pub proposal : Account<'info,StateProposalPda>,
    #[account(
        mut,
        seeds=[
            STATE_SEEDS,
            state_name.as_ref(),
            country.key().as_ref()
        ],
        bump= state.bump,
    )]
    pub state : Account<'info,State>,

    #[account(
        init,
        payer = signer,
        seeds=[
            STATE_AUTHORITY,
            country.key().as_ref(),
            state_authority.key().as_ref()
        ],
        bump,
        space = 8 + StateAuthority::SIZE
    )]
    pub state_authority_receipt : Account<'info,StateAuthority>,

    pub system_program: Program<'info,System>,

}



pub fn add_state_authority(ctx:Context<AddStateAuthority>,_country_name: [u8;32],_state_name: [u8;32])->Result<()>{


    let state_receipt = &mut ctx.accounts.state_authority_receipt;

    let state = &mut ctx.accounts.state;

    state_receipt.state_pubkey = state.key();

    state_receipt.bump = ctx.bumps.state_authority_receipt;

    require!(state.total_authorities < ctx.accounts.proposal.state_total_authorities,ErrorCode::AuthorityLimitReached);

    state.total_authorities = state.total_authorities + 1; 

    Ok(())

}

