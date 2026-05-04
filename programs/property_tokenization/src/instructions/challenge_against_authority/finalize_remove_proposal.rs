use anchor_lang::prelude::*;

use crate::{common::ProposalStatus, errors::ErrorCode, state::ElectAuthority};



#[derive(Accounts)]
pub struct FinalizeRemoveAuthority<'info>{


    #[account(
        mut,
    )]
    pub signer:Signer<'info>,

    #[account(
        mut,
        constraint = remove_proposal.status == ProposalStatus::Active @ ErrorCode::ProposalNotActive
    )]
    pub remove_proposal: Account<'info,ElectAuthority>,

}



pub fn finalize_remove_proposal(
    ctx:Context<FinalizeRemoveAuthority>
)->Result<()>{

    let proposal = &mut ctx.accounts.remove_proposal;


    let now = Clock::get()?.unix_timestamp;

    require!(proposal.voting_for_authority_deadline < now ,ErrorCode::VotingStillActive );

    require!( now < proposal.add_new_authority_deadline,ErrorCode::AuthorityAddDeadline);

    if proposal.rm_total_voting_power_gained > proposal.rm_voting_threshold{

            proposal.status =ProposalStatus::Approved;

    }
    if proposal.rm_total_voting_power_gained < proposal.rm_voting_threshold {
        proposal.status = ProposalStatus::Rejected;
    }


    Ok(())
}