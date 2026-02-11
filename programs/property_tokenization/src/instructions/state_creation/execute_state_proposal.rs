use anchor_lang::prelude::*;
use crate::state::{Country, State, StateProposalPda};
use crate::errors::ErrorCode::{self};
const PROPOSAL_SEEDS: &[u8] = b"proposal";
const STATE_SEEDS : &[u8] = b"state";
#[derive(Accounts)]

pub struct ExecuteStatePda<'info>{

    #[account{
        mut,
        seeds=[
            PROPOSAL_SEEDS,
            &proposal.state_id.to_le_bytes(),
            country.key().as_ref()
        ],
        bump = proposal.bump,
        constraint = proposal.approved @ ErrorCode::ProposalNotApproved,
        close = signer
    }]
    pub proposal : Account<'info,StateProposalPda>,

    #[account(
        constraint = country.authority.contains(&signer.key()) @ ErrorCode::InvalidCountry
    )]
    pub country : Account<'info,Country>,

    #[account(
        mut,
        constraint = proposal.approval.contains(&signer.key()) @ ErrorCode::NotAuthorized,
    )]
    pub signer : Signer<'info>,

    #[account(
        init,
        payer = signer,
        seeds = [
            STATE_SEEDS,
            &proposal.state_id.to_le_bytes(),
            country.key().as_ref(),
        ],
        bump,
        space = 8 + State::SIZE
    )]

    pub state : Account<'info,State>,

    pub system_program : Program<'info,System>,


}

pub fn execute_state_proposal(
    ctx: Context<ExecuteStatePda>
)->Result<()>{

    let state = &mut ctx.accounts.state;

    let proposal = &mut ctx.accounts.proposal;

    let country = & ctx.accounts.country;

    state.state_id = proposal.state_id;

    state.state_name = proposal.state_name.clone();

    state.authorities = proposal.state_authorities.clone();

    state.threshold = proposal.state_authority_threshold;

    state.country_id = country.country_id;

    state.country_pubkey = proposal.country_pubkey;

    state.bump = ctx.bumps.state;

    proposal.executed = true;

    Ok(())
}