use anchor_lang::prelude::*;

use crate::{common::RT_CHG_PROPOSAL_SEEDS, functions::finalize, state::RTChgProposal};



#[derive(Accounts)]
#[instruction(proposal_id:u64,property_system_account:Pubkey)]
pub struct RTChgFinalize<'info>{


    #[account()]
    pub signer:Signer<'info>,


    #[account(
        seeds=[
            RT_CHG_PROPOSAL_SEEDS,
            property_system_account.as_ref(),
            &proposal_id.to_le_bytes(),
            
        ],
        bump = proposal.bump
    )]
    pub proposal : Account<'info,RTChgProposal>
}


pub fn finalize_rtc_proposal(ctx:Context<RTChgFinalize>)->Result<()>{

    let proposal = &mut *ctx.accounts.proposal; 

    finalize(proposal)?;

    

    Ok(())
}