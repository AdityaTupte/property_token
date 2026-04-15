use anchor_lang:: prelude::*;

use crate::{common::{COUNTRY_AUTHORITY, STATE_APPROVE_RECEIPT, STATE_PROPOSAL_SEEDS}, errors::ErrorCode, state::{Country, CountryAuthority, StateProposalAprroveReceipt, StateProposalPda}};


#[derive(Accounts)]
pub struct ApproveState<'info>{

    #[account(
        mut,
        seeds =[
            STATE_PROPOSAL_SEEDS,
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
       mut,
    )]
    pub signer: Signer<'info>,


    #[account(
        seeds=[
            COUNTRY_AUTHORITY,
            country.key().as_ref(),
            signer.key().as_ref()
        ],
        bump,
    )]
    pub country_authority:Account<'info,CountryAuthority>,

    #[account(
        init,
        payer= signer,
        seeds=[
            STATE_APPROVE_RECEIPT,
            proposal.key().as_ref(),
            signer.key().as_ref()
        ],
        bump,
        space = 8 + StateProposalAprroveReceipt::SIZE
    )]
    pub state_creation_recepit : Account<'info,StateProposalAprroveReceipt>,

    pub system_program:Program<'info,System>,
}


pub fn approve_state(
    ctx:Context<ApproveState>
)->Result<()>{

    let proposal = &mut ctx.accounts.proposal;

    let country = &mut ctx.accounts.country;

    let state_creation_recepit = &mut ctx.accounts.state_creation_recepit;
    
    state_creation_recepit.proposal = proposal.key();

    state_creation_recepit.bump = ctx.bumps.state_creation_recepit;

    proposal.approval += 1;

    if proposal.approval >= country.threshold {

        proposal.approved = true;

    } 
    // require!(!proposal.approval.contains(&signer.key()) , ErrorCode::AuthorityApproved );

    // proposal.approval.push(signer.key());

    // if proposal.approval.len() >= country.threshold as usize {

    //     proposal.approved = true;

    

    Ok(())
}