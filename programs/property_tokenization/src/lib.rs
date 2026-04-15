use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;
pub mod events;
pub mod errors;
pub mod constant;
pub mod common;
pub mod functions;
use crate::instructions::*;

declare_id!("BD4CURALcvWKHuLrcx74iaJaaUrKbRogXYXcNBPfqFRR");

#[program]
pub mod property_tokenization {

    use super::*;


    pub fn create_property_system (
        ctx:Context<PropertySystem>,
        system_id : u64,decimal:u8,number_of_tokens:u64,
        safety_threshold:u8,
        trustee_salary_threshold:u8,
        arbitrator_salary_threshold:u8,
        dividend_threshold:u8,
        reinvestment_threshold:u8,
    )->Result<()>{

        create_property_system_account::create(
            ctx,
            system_id,
            decimal, 
            number_of_tokens, 
            safety_threshold, 
            trustee_salary_threshold, 
            arbitrator_salary_threshold, 
            dividend_threshold, 
            reinvestment_threshold
        )?;


        Ok(())
    }
                 //COUNTRYCREATIONAUTHORIY
    pub fn create_approve_country_authority(
        ctx:Context<CreateApproveCountryAuthority>,
        threshold : u8,
        authority: Vec<Pubkey>,
    )->Result<()>{

        country_creation::create_approve_country_authority(ctx, threshold, authority)?;


        Ok(())

    }

            
    //           //  COUNTRYCREATION
    pub fn create_country_proposal(
    ctx:Context<ProposeCountry>,
    country_name: [u8;32],
    country_id: u16,
    
    total_authority:u8,
    
    //authority: Vec<Pubkey>,
    country_pda_threshold: u8,
    )->Result<()>{

        country_creation::create_country_proposal(ctx, country_name, country_id, total_authority, country_pda_threshold)?;
        Ok(())
    }

    pub fn approve_country(
        ctx:Context<ApproveCountry>,
        country_name: [u8;32],
    )->Result<()>{

        country_creation::approve_country(ctx, country_name)?;
        Ok(())
    }

    pub fn execute_country_propsal( 
        ctx:Context<ExecuteCountryPda>,
        country_name: [u8;32],
    )->Result<()>{

        country_creation::execute_country_propsal(ctx, country_name)?;
        Ok(())
    }


    pub fn add_country_authority(
        ctx:Context<AddCountryAuthority>,country_name: [u8;32],
    )->Result<()>{
        country_creation::add_country_authority(ctx,country_name)?;
        Ok(())
    }

    // //             //STATECREATION


    pub fn state_creation_proposal(
            ctx:Context<StateProposal>,
             state_name: [u8;32],
              country_name: [u8;32],
    state_id : u16,
   
    //state__total_authorities : Vec<Pubkey>,
    state_total_authorities :u8,
    state_authority_threshold: u8,
   
    )->Result<()>{

        state_creation::create_state_proposal(ctx, state_name, country_name, state_id, state_total_authorities, state_authority_threshold)?;

        Ok(())


    }


}
