use anchor_lang::prelude::*;


use crate::common::SELLPROPERTY;
use crate::functions::finalize;
use crate::instructions::property_system;
use crate::state::PropertySellProposal;




#[derive(Accounts)]
#[instruction(proposal_id:u64,property_system_account:Pubkey)]
pub struct Finalize<'info>{


    #[account()]
    pub signer:Signer<'info>,


    #[account(
        mut,
        seeds=[
            SELLPROPERTY,
            property_system_account.as_ref(),
            &proposal_id.to_le_bytes(),
        ],
        bump = proposal.bump
    )]
    pub proposal : Account<'info,PropertySellProposal>
}


pub fn finalize_sell_proposal(ctx:Context<Finalize>,_proposal_id:u64,_property_system_account:Pubkey)->Result<()>{

    let proposal = &mut *ctx.accounts.proposal; 

    finalize(proposal)?;

    Ok(())
}
