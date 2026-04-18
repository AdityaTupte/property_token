use anchor_lang::prelude::*;
use crate::common::{COUNTRY_SEED, STATE_PROPOSAL_SEEDS, STATE_SEEDS};
use crate::state::{Country, State, StateProposalPda};
use crate::errors::ErrorCode::{self};

#[derive(Accounts)]
#[instruction(state_name:[u8;32],country_name:[u8;32])]
pub struct ExecuteStatePda<'info>{

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
        init,
        payer = signer,
        seeds = [
            STATE_SEEDS,
            state_name.as_ref(),
            country.key().as_ref(),
        ],
        bump,
        space = 8 + State::SIZE
    )]

    pub state : Account<'info,State>,

    pub system_program : Program<'info,System>,


}

pub fn execute_state_proposal(
    ctx: Context<ExecuteStatePda>,
    _state_name:[u8;32],
    _country_name : [u8;32]
)->Result<()>{

    let state = &mut ctx.accounts.state;

    let proposal = &mut ctx.accounts.proposal;

    let country = & ctx.accounts.country;

    state.state_id = proposal.state_id;

    state.state_name = proposal.state_name;

    // state.total_authorities = proposal.state_total_authorities;

    state.threshold = proposal.state_authority_threshold;

    state.country_id = country.country_id;

    state.country_pubkey = proposal.country_pubkey;

    state.bump = ctx.bumps.state;

    proposal.executed = true;

    Ok(())
}