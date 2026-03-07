use anchor_lang::prelude::*;

use crate::{ common::SAFETYPROPOSAL, functions::finalize, state::SafetyProposal};


#[derive(Accounts)]
pub struct Finalize<'info>{

    #[account()]
    pub signer:Signer<'info>,


    #[account(
        seeds=[
            SAFETYPROPOSAL,
            proposal.property_system.as_ref(),
            &proposal.proposal_id.to_le_bytes(),
        ],
        bump = proposal.bump
    )]
    pub proposal : Account<'info,SafetyProposal>
}


pub fn finalize_sell_proposal(ctx:Context<Finalize>)->Result<()>{

    let proposal = &mut *ctx.accounts.proposal; 

    finalize(proposal)?;

    Ok(())
}