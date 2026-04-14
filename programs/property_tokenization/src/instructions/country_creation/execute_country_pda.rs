use anchor_lang::prelude::*;

use crate::{common::{COUNTRY_PROPOSAL_SEEDS, COUNTRY_SEED}, errors::ErrorCode, state::{Country, ProposalCountryPda}};

#[derive(Accounts)]
#[instruction(country_name:String)]
pub struct ExecuteCountryPda<'info>{

    #[account(
        mut,
        seeds=[
        COUNTRY_PROPOSAL_SEEDS,
        country_name.as_ref(),
    ],
    bump,
        constraint = proposal.approved @ ErrorCode::ProposalNotApproved,
    )]
    pub proposal : Account<'info,ProposalCountryPda>,

    #[account(mut)]
    pub signer:Signer<'info>,

    #[account(
        init,
        payer = signer,
        seeds = [
            COUNTRY_SEED,
            proposal.country_name.as_ref(),
        ],
        bump,
        space = 8 +  Country::SIZE
    )]

    pub country_pda : Account<'info,Country>,

    pub system_program : Program<'info,System>,

}


pub fn execute_country_propsal(
    ctx:Context<ExecuteCountryPda>,
    _country_name:String
)->Result<()>{

    let proposal = &mut ctx.accounts.proposal;

    let country = &mut ctx.accounts.country_pda;

    country.country_id = proposal.country_id;
    
    country.country_name = proposal.country_name.clone();

    // country.total_authority = proposal.total_authority;

    country.threshold = proposal.country_pda_threshold;

    country.bump = ctx.bumps.country_pda;

    Ok(())


}