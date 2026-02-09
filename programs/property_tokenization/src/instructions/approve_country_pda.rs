
use anchor_lang::prelude::*;

use crate::{errors::ErrorCode, state::*};


#[derive(Accounts)]

pub struct ApproveCountryId<'info>{

   #[account(
    mut,
    constraint = !proposal.executed @ ErrorCode::AlreadyApproved
   )]
    pub proposal : Account<'info,ProposalCountryPda>,

    #[account(
        seeds = [
            b"AuthorityForApprovingCountry",
        ],
        bump = authority.bump, 
    )]
    pub authority: Account<'info,ApproveCountryAuthority>,

    #[account(
        mut,
        constraint = authority.authority.contains(&signer.key()) @ ErrorCode::NotAuthorized,
    )]

    pub signer: Signer<'info>,

}

pub fn approve_country(
    ctx:Context<ApproveCountryId>,
)->Result<()>{

    let proposal = &mut ctx.accounts.proposal;
    let authority = &mut ctx.accounts.authority;
    let signer  = &mut ctx.accounts.signer;

    require!(!proposal.approvals.contains(&signer.key()), ErrorCode::AuthorityApproved );

    proposal.approvals.push(signer.key());

    if proposal.approvals.len() >= authority.threshold as usize {
        proposal.executed = true;
    }

    Ok(())
}