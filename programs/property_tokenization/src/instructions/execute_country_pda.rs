use anchor_lang::prelude::*;

use crate::{errors::ErrorCode, state::{Country, ProposalCountryPda}};
const  COUNTRY_SEED : &[u8] = b"country";
#[derive(Accounts)]

pub struct ExecuteCountryPda<'info>{

    #[account(
        mut,
        constraint = proposal.approved @ ErrorCode::ProposalNotApproved,
        constraint = proposal.approvals.contains(&signer.key()),
        close = signer
    )]
    pub proposal : Account<'info,ProposalCountryPda>,

    #[account(mut)]
    pub signer:Signer<'info>,

    #[account(
        init,
        payer = signer,
        seeds = [
            COUNTRY_SEED,
            &proposal.country_id.to_le_bytes(),
        ],
        bump,
        space = 8 +  Country::SIZE
    )]

    pub country_pda : Account<'info,Country>,

    pub system_program : Program<'info,System>,

}


pub fn execute_country_propsal(
    ctx:Context<ExecuteCountryPda>,
)->Result<()>{

    let proposal = &mut ctx.accounts.proposal;

    let country = &mut ctx.accounts.country_pda;

    country.country_id = proposal.country_id;
    
    country.country_name = proposal.country_name.clone();

    country.authority = proposal.authority.clone();

    ////threashold additoon fpor country approval 

    


    Ok(())


}