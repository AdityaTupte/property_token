use anchor_lang::prelude::*;

use crate::{ common::SAFETYPROPOSAL, functions::finalize, state::SafetyProposal};


#[derive(Accounts)]
#[instruction(proposal_id:u64,property_system:Pubkey)]
pub struct Finalize<'info>{

    #[account()]
    pub signer:Signer<'info>,


    #[account(
        mut,
        seeds=[
            SAFETYPROPOSAL,
            property_system.as_ref(),
            &proposal_id.to_le_bytes(),
        ],
        bump = proposal.bump
    )]
    pub proposal : Account<'info,SafetyProposal>
}


pub fn finalize_safety_proposal(
    ctx:Context<Finalize>,
    _proposal_id:u64,_property_system:Pubkey
)->Result<()>{

    let proposal = &mut *ctx.accounts.proposal; 

    finalize(proposal)?;

    Ok(())
}