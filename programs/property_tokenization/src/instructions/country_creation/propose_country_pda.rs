use std::collections::BTreeSet;

use anchor_lang::prelude::*;

use crate::{errors::ErrorCode, state::ProposalCountryPda};

const PROPOSAL_SEEDS: &[u8] = b"proposal";

#[derive(Accounts)]
#[instruction(country_id:u16)]
pub struct ProposeCountry<'info>{

    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        payer = signer,
        seeds =[ 
            PROPOSAL_SEEDS,
            country_id.to_le_bytes().as_ref(),
        ],
        bump,
        space = 8 + ProposalCountryPda::SIZE
    )]

    pub country_acc : Account<'info,ProposalCountryPda>,

    pub system_program : Program<'info,System>,

}

pub fn create_country_proposal(
    ctx:Context<ProposeCountry>,
    country_id: u16,
    country_name: String,
    authority: Vec<Pubkey>,
    country_pda_threshold: u8,
) -> Result<()>{

        require_eq!(10,authority.len(), ErrorCode::ApproveAuthorityInvalid);

        require!(country_name.len() > 0 && country_name.len() <= 32,ErrorCode::CountryNameInvalid);

        require!( country_pda_threshold > 1 && country_pda_threshold <= 10 , ErrorCode::CountryPdaThresholdInvalid);
       
        let unique: BTreeSet<Pubkey> = authority.iter().cloned().collect();

        require!( unique.len() == authority.len(),ErrorCode::DuplicateAuthority);
        
        let proposalcountry  = &mut ctx.accounts.country_acc;

        proposalcountry.country_id = country_id;

        proposalcountry.country_name = country_name.to_uppercase();
        
        proposalcountry.authority = authority;

        proposalcountry.country_pda_threshold = country_pda_threshold;
        
        proposalcountry.approved = false;

        proposalcountry.executed = false;

        proposalcountry.bump = ctx.bumps.country_acc;

    Ok(())
}


