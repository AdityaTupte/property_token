use anchor_lang:: prelude::*;

use crate::{common::{COUNTRY_AUTHORITY, COUNTRY_SEED, STATE_APPROVE_RECEIPT, STATE_PROPOSAL_SEEDS}, errors::ErrorCode, state::{Country, CountryAuthority, StateProposalAprroveReceipt, StateProposalPda}};


#[derive(Accounts)]
#[instruction(
    state_name:[u8;32],
    country_name:[u8;32])]
pub struct ApproveState<'info>{

    #[account(
        mut,
        seeds =[
            STATE_PROPOSAL_SEEDS,
            state_name.as_ref(),
            country.key().as_ref(),
        ],
        bump = proposal.bump,
        constraint = !proposal.approved @ ErrorCode::AlreadyApproved,
    )]
    pub proposal: Account<'info,StateProposalPda>,
    
    #[account(
        seeds = [
            COUNTRY_SEED,
            country_name.as_ref()
            ],
        bump = country.bump,
        constraint = proposal.country_pubkey == country.key() @ ErrorCode::InvalidCountry
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
    ctx:Context<ApproveState>,
    _state_name:[u8;32],
    _country_name:[u8;32]
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