use anchor_lang::prelude::*;


use crate::common::{BUYPROPERTY};
use crate::functions::finalize;
use crate::state::PropertyBuyProposal;




#[derive(Accounts)]
#[instruction(proposal_id:u64,property_system_account:Pubkey)]
pub struct BuyProposalFinalize<'info>{


    #[account()]
    pub signer:Signer<'info>,


    #[account(
        mut,
        seeds=[
            BUYPROPERTY,
            property_system_account.as_ref(),
            &proposal_id.to_le_bytes(),
        ],
        bump = proposal.bump
    )]
    pub proposal : Account<'info,PropertyBuyProposal>
}


pub fn finalize_buy_proposal(
    ctx:Context<BuyProposalFinalize>,
    _proposal_id:u64,
    _property_system_account:Pubkey
)->Result<()>{

    let proposal = &mut *ctx.accounts.proposal; 

    finalize(proposal)?;

    Ok(())
}