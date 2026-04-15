

use anchor_lang::prelude::*;

use crate::{common::COUNTRY_PROPOSAL_SEEDS, errors::ErrorCode, state::ProposalCountryPda};



#[derive(Accounts)]
#[instruction(country_name:[u8;32])]
pub struct ProposeCountry<'info>{

    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        payer = signer,
        seeds =[ 
            COUNTRY_PROPOSAL_SEEDS,
            country_name.as_ref(),
            // country_id.to_le_bytes().as_ref(),
        ],
        bump,
        space = 8 + ProposalCountryPda::SIZE
    )]

    pub country_acc : Account<'info,ProposalCountryPda>,

    pub system_program : Program<'info,System>,

}

pub fn create_country_proposal(
    ctx:Context<ProposeCountry>,
    country_name: [u8;32],
    country_id: u16,
    total_authority:u8,
    country_pda_threshold: u8,
) -> Result<()>{

        
           
        require!(country_name.len() > 0 && country_name.len() <= 32,ErrorCode::CountryNameInvalid);

        require!( country_pda_threshold > 1 && country_pda_threshold <= 10 , ErrorCode::CountryPdaThresholdInvalid);
       
        require!(
        country_name.iter().any(|&c| c != 0),
        ErrorCode::CountryNameInvalid
    );
    require!(
        country_name.iter().all(|&c| c == 0 || (c >= b'A' && c <= b'Z')),
        ErrorCode::NotInUppercase
    );
      
        
        let proposalcountry  = &mut ctx.accounts.country_acc;

        proposalcountry.country_id = country_id;

        proposalcountry.country_name = country_name;
        
        proposalcountry.total_authority = total_authority;

        proposalcountry.country_pda_threshold = country_pda_threshold;
        
        proposalcountry.approved = false;

        proposalcountry.executed = false;

        proposalcountry.bump = ctx.bumps.country_acc;

    Ok(())
}

