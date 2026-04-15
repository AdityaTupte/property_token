use anchor_lang::prelude::*;

use crate::{common::{COUNTRY_APPROVE_AUTHORITY_SEEDS, COUNTRY_AUTHORITY, COUNTRY_PROPOSAL_SEEDS, COUNTRY_SEED}, errors::ErrorCode, state::{ApproveCountryAuthority, Country, CountryAuthority, ProposalCountryPda}};

#[derive(Accounts)]
#[instruction(country_name: [u8;32])]
pub struct AddCountryAuthority<'info>{

     #[account(
        seeds = [
            COUNTRY_APPROVE_AUTHORITY_SEEDS,
        ],
        bump = authority.bump, 
    )]
    pub authority: Account<'info,ApproveCountryAuthority>,

    #[account(
        mut,
        constraint = authority.authority.contains(&signer.key()) @ ErrorCode::NotAuthorized
    )]
    pub signer : Signer<'info>,

    pub country_authority : SystemAccount<'info>,


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

    #[account(
        mut,
        seeds=[
            COUNTRY_SEED,
            country_name.as_ref(),
        ],
        bump= country.bump,
    )]
    pub country : Account<'info,Country>,

    #[account(
        init,
        payer = signer,
        seeds=[
            COUNTRY_AUTHORITY,
            country.key().as_ref(),
            country_authority.key().as_ref()
        ],
        bump,
        space = 8 + CountryAuthority::SIZE
    )]
    pub country_receipt : Account<'info,CountryAuthority>,

    pub system_program: Program<'info,System>,

}



pub fn add_country_authority(ctx:Context<AddCountryAuthority>,_country_name: [u8;32],)->Result<()>{


    let country_receipt = &mut ctx.accounts.country_receipt;

    let country = &mut ctx.accounts.country;

    country_receipt.country_pda = country.key();

    country_receipt.bump = ctx.bumps.country_receipt;

    require!(country.total_authority < ctx.accounts.proposal.total_authority,ErrorCode::AuhtorityLimitReached);

    country.total_authority = country.total_authority + 1; 

    Ok(())



}