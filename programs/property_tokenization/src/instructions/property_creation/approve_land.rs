use anchor_lang::prelude::*;

use crate::common::{PROPERTY_PROPOSAL_SEEDS, STATE_SEEDS};
use crate::state::{PropertyProposal, State};

use crate::errors::ErrorCode::{self};


#[derive(Accounts)]

pub struct ApproveLand<'info>{



    #[account(
        seeds=[
            STATE_SEEDS,
            &state.state_id.to_le_bytes(),
            state.country_pubkey.as_ref()
        ],
        bump = state.bump,
       
    )]
    pub state: Account<'info,State>,


    #[account(
        mut,
        seeds=[
            PROPERTY_PROPOSAL_SEEDS,
            &property_proposal.property_id.to_le_bytes(),
            state.key().as_ref(),
            ],
        bump = property_proposal.bump,
        constraint = !property_proposal.approved @ ErrorCode::AlreadyApproved,
        constraint = property_proposal.state_pubkey == state.key() @ ErrorCode::InvalidProperty
    )]

    pub property_proposal : Account<'info,PropertyProposal>,

    #[account(
        mut,
        constraint = state.authorities.contains(&signer.key()) @ ErrorCode::NotAuthorized,
    )]
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
