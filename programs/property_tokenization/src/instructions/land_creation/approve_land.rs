use anchor_lang::prelude::*;

use crate::state::{Country, LandProposal, State};

use crate::errors::ErrorCode::{self};

const PROPOSAL_SEEDS: &[u8] = b"proposal";

const STATE_SEEDS : &[u8] = b"state";

const  COUNTRY_SEED : &[u8] = b"country";

#[derive(Accounts)]

pub struct ApproveLand<'info>{

    #[account(
        seeds = [
            COUNTRY_SEED,
            &country.country_id.to_le_bytes(),
        ],
        bump = country.bump
    )]
    pub country : Account<'info,Country>,

    #[account(
        seeds=[
            STATE_SEEDS,
            &state.state_id.to_le_bytes(),
            country.key().as_ref()
        ],
        bump = state.bump,
        constraint = state.authorities.contains(&signer.key()) @ ErrorCode::NotAuthorized,
        constraint = country.key() == state.country_pubkey @ ErrorCode::InvalidCountry
    )]
    pub state: Account<'info,State>,


    #[account(
        mut,
        seeds=[
            PROPOSAL_SEEDS,
            &land_proposal.land_id.to_le_bytes(),
            state.key().as_ref(),
            country.key().as_ref(),  
            ],
        bump = land_proposal.bump,
        constraint = !land_proposal.approved @ ErrorCode::AlreadyApproved,
        constraint = land_proposal.state_pubkey == state.key() @ ErrorCode::InvalidLand
    )]

    pub land_proposal : Account<'info,LandProposal>,

    #[account(mut)]
    pub signer : Signer<'info>

}

    pub fn approve(
        ctx:Context<ApproveLand>
    )->Result<()>{

        let proposal = &mut ctx.accounts.land_proposal ;

        require!(!proposal.approval.contains(&ctx.accounts.signer.key()),ErrorCode::AuthorityApproved);

        proposal.approval.push(ctx.accounts.signer.key());

        if proposal.approval.len() >= ctx.accounts.state.threshold as usize {
            
            proposal.approved = true ;
        }

        Ok(())
        
    }
