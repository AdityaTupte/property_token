use anchor_lang::prelude::*;

use crate::constant::ProposalStatus;
use crate::state::PropertySellProposal;
use crate::errors::ErrorCode;



#[derive(Accounts)]
pub struct Finalize<'info>{


    #[account()]
    pub signer:Signer<'info>,


    #[account(
        seeds=[
            SELLPROPERTY,
            proposal.property_system_account.as_ref(),
            &proposal.proposal_id.to_le_bytes(),
        ],
        bump = proposal.bump
    )]
    pub proposal : Account<'info,PropertySellProposal>
}


pub fn finalize_sell_proposal(ctx:Context<Finalize>)->Result<()>{

    let proposal = &mut ctx.accounts.proposal; 

    let current_time= Clock::get()?.unix_timestamp;

    require!(current_time > proposal.end_time && proposal.status == Active ,ErrorCode::CannotFinalize);

    proposal.status = ProposalStatus::Failed;

    Ok(())
}