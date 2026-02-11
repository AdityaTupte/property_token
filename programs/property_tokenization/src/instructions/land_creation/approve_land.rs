use anchor_lang::prelude::*;

use crate::state::{Country, State};

const PROPOSAL_SEEDS: &[u8] = b"proposal";

const STATE_SEEDS : &[u8] = b"state";

#[derive(Accounts)]

pub struct ApproveLand<'info>{

    #[account()]
    pub country : Account<'info,Country>,

    #[account(
        seeds=[
            STATE_SEEDS,
            &state.state_id.to_le_bytes(),
            country.key().as_ref()
        ],
        bump = state.bump,
        constraint = 
    )]
    pub state: Account<'info,State>,

    #[account(mut)]
    pub signer : Signer<'info>

}