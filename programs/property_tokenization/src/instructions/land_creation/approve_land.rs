use anchor_lang::prelude::*;

use crate::state::{Country, PropertyProposal, State};

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
            &property_proposal.property_id.to_le_bytes(),
            state.key().as_ref(),
            country.key().as_ref(),  
            ],
        bump = property_proposal.bump,
        constraint = !property_proposal.approved @ ErrorCode::AlreadyApproved,
        constraint = property_proposal.state_pubkey == state.key() @ ErrorCode::InvalidProperty
    )]

    pub property_proposal : Account<'info,PropertyProposal>,

    #[account(mut)]
    pub signer : Signer<'info>

}

    pub fn approve(
        ctx:Context<ApproveLand>
    )->Result<()>{

        let proposal = &mut ctx.accounts.property_proposal ;

        require!(!proposal.approval.contains(&ctx.accounts.signer.key()),ErrorCode::AuthorityApproved);

        proposal.approval.push(ctx.accounts.signer.key());

        if proposal.approval.len() >= ctx.accounts.state.threshold as usize {
            
            proposal.approved = true ;
        }

        Ok(())
        
    }
