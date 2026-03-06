
use anchor_lang::prelude::*;

use crate::{common::{ COUNTRY_APPROVE_AUTHORITY_SEEDS}, errors::ErrorCode, state::*};


#[derive(Accounts)]

pub struct ApproveCountryId<'info>{

   #[account(
    mut,
    constraint = !proposal.approved @ ErrorCode::AlreadyApproved
   )]
    pub proposal : Account<'info,ProposalCountryPda>,

    #[account(
        seeds = [
            COUNTRY_APPROVE_AUTHORITY_SEEDS,
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
        proposal.approved = true;
    }

    Ok(())
}