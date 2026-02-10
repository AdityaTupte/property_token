use anchor_lang:: prelude::*;

use crate::{errors::ErrorCode, state::{Country, StateProposalPda}};

const PROPOSAL_SEEDS: &[u8] = b"proposal";
#[derive(Accounts)]
pub struct ApproveState<'info>{

    #[account(
        mut,
        seeds =[
            PROPOSAL_SEEDS,
            &proposal.state_id.to_le_bytes(),
            country.key().as_ref(),
        ],
        bump = proposal.bump,
        constraint = !proposal.approved @ ErrorCode::AlreadyApproved,
    )]
    pub proposal: Account<'info,StateProposalPda>,
    
    #[account(
        constraint = proposal.country_id == country.country_id @ ErrorCode::InvalidCountry
    )]
    pub country: Account<'info,Country>,
    
    #[account(
        constraint = country.authority.contains(&signer.key())
    )]
    pub signer: Signer<'info>,
    
}


pub fn approve(
    ctx:Context<ApproveState>
)->Result<()>{

    let proposal = &mut ctx.accounts.proposal;

    let country = &mut ctx.accounts.country;

    let signer = & ctx.accounts.signer;

    require!(!proposal.approval.contains(&signer.key()) , ErrorCode::AuthorityApproved );

    proposal.approval.push(signer.key());

    if proposal.approval.len() >= country.threshold as usize {

        proposal.approved = true;

    }

    Ok(())
}