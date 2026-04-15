
use anchor_lang::prelude::*;

use crate::{common::{ COUNTRY_APPROVE_AUTHORITY_SEEDS, COUNTRY_CREATION_AUHTORITY, COUNTRY_PROPOSAL_SEEDS}, errors::ErrorCode, state::*};


#[derive(Accounts)]
#[instruction(country_name:[u8;32])]
pub struct ApproveCountry<'info>{

   #[account(
    mut,
    seeds=[
        COUNTRY_PROPOSAL_SEEDS,
        country_name.as_ref(),
    ],
    bump,
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
        init,
        payer=signer,
        seeds=[
            COUNTRY_CREATION_AUHTORITY,
            proposal.key().as_ref(),
            signer.key().as_ref()
        ],
        bump,
        space = 8 + ApproveCountryAuthorityReceipt::SIZE
    )]
    pub authority_recipt : Account<'info,ApproveCountryAuthorityReceipt>,

    #[account(
        mut,
        constraint = authority.authority.contains(&signer.key()) @ ErrorCode::NotAuthorized,
    )]

    pub signer: Signer<'info>,

    pub system_program:Program<'info,System>,

}

pub fn approve_country(
    ctx:Context<ApproveCountry>,
    _country_name:[u8;32]
)->Result<()>{

    let proposal = &mut ctx.accounts.proposal;
    let authority = &mut ctx.accounts.authority;
    
    let authority_recipt = &mut ctx.accounts.authority_recipt;

    authority_recipt.country_proposal = proposal.key() ;

    authority_recipt.bump = ctx.bumps.authority_recipt;

    proposal.approvals += 1;

    if proposal.approvals >= authority.threshold {

        proposal.approved = true;

    } 

    // require!(!proposal.approvals.contains(&signer.key()), ErrorCode::AuthorityApproved );

    // proposal.approvals.push(signer.key());

    // if proposal.approvals.len() >= authority.threshold as usize {
    //     proposal.approved = true;
    // }

    Ok(())
}