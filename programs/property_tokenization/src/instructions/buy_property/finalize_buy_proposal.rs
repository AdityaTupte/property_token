use anchor_lang::prelude::*;

use crate::functions::finalize;
use crate::state::PropertyBuyProposal;




#[derive(Accounts)]
pub struct Finalize<'info>{


    #[account()]
    pub signer:Signer<'info>,


    #[account(
        seeds=[
            SELLPROPERTY,
            proposal.buyer.as_ref(),
            &proposal.proposal_id.to_le_bytes(),
        ],
        bump = proposal.bump
    )]
    pub proposal : Account<'info,PropertyBuyProposal>
}


pub fn finalize_sell_proposal(ctx:Context<Finalize>)->Result<()>{

    let proposal = &mut *ctx.accounts.proposal; 

    finalize(proposal)?;

    Ok(())
}