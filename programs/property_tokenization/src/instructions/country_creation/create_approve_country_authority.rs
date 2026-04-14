
use anchor_lang::prelude::*;

use crate::{common::COUNTRY_APPROVE_AUTHORITY_SEEDS, errors::ErrorCode, functions::assert_unique_owners, state::ApproveCountryAuthority};
const HARDCODED_PUBKEY: Pubkey = pubkey!("EezSke1hYxfEZuCrxDAgGe1nb7i2DoqJnaFJQPBPRpJs");
#[derive(Accounts)]

pub struct CreateApproveCountryAuthority<'info>{

    #[account(
        mut,
        address = HARDCODED_PUBKEY,
    )]

    pub signer : Signer<'info>,

    #[account(
        init,
        payer = signer,
        seeds = [
           COUNTRY_APPROVE_AUTHORITY_SEEDS
        ],
        bump,
        space = 8 + ApproveCountryAuthority::SIZE,
    )]

    pub authority : Account<'info,ApproveCountryAuthority>,

    pub system_program : Program<'info,System>

}

pub fn create_approve_country_authority(
    ctx:Context<CreateApproveCountryAuthority>,
    threshold : u8,
    authority: Vec<Pubkey>,
)->Result<()>{

    require_eq!(10,authority.len(), ErrorCode::ApproveAuthorityInvalid);

    require!( 0 < threshold && threshold <= 10 , ErrorCode::ApproveAuthorityThresholdInvalid);

    let approve_authority = &mut ctx.accounts.authority;
    
    assert_unique_owners(&authority)?;

    require!( authority.len() == authority.len(),ErrorCode::DuplicateAuthority);

    approve_authority.threshold = threshold;

    approve_authority.authority = authority;

    approve_authority.bump = ctx.bumps.authority;

    Ok(())

}