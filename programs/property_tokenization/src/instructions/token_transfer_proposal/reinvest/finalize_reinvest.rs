use anchor_lang::prelude::*;

use crate::{common::USEREINVESTMENTOKEN, functions::finalize, state::{TokenTransferProposal}};


#[derive(Accounts)]
#[instruction(proposal_id:u64,property_system:Pubkey)]
pub struct ReinvestFinalize<'info>{

    #[account()]
    pub signer:Signer<'info>,


    #[account(
        seeds=[
            USEREINVESTMENTOKEN,
            property_system.as_ref(),
            &proposal_id.to_le_bytes(),
        ],
        bump = proposal.bump
    )]
    pub proposal : Account<'info,TokenTransferProposal>
}


pub fn finalize_reinvest_proposal(
    ctx:Context<ReinvestFinalize>,
    proposal_id:u64,property_system:Pubkey
)->Result<()>{

    let proposal = &mut *ctx.accounts.proposal; 

    finalize(proposal)?;

    Ok(())
}