use anchor_lang::prelude::*;

use crate::state::{Country, StateProposalPda};
use crate::errors::ErrorCode::{self};
const PROPOSAL_SEEDS: &[u8] = b"proposal";
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
    }]
    pub proposal : Account<'info,StateProposalPda>,

    #[account()]
    pub country : Account<'info,Country>,

    
}