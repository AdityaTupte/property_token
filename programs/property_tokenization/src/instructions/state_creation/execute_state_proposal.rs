use anchor_lang::prelude::*;
use anchor_spl::token_2022::spl_token_2022::extension::AccountType;

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
    }]
    pub proposal : Account<'info,StateProposalPda>,

    #[account()]
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