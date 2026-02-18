use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;
pub mod events;
pub mod errors;
pub mod constant;
pub mod functions;
use crate::instructions::*;

declare_id!("8PRrAQa9Y4ZVxcdNXP4Q8CiQzAyiWoyJT64t7FUmrN4L");

#[program]
pub mod peoperty_tokenization {

    use super::*;

    pub fn create_property_system_account(
        ctx:Context<PropertySystemAcc>,
        system_id:u64,
        decimal:u8,
        amount:u64,
        safety_threshold:u8,
        trustee_salary_threshold:u8,
        arbitrator_salary_threshold:u8,
        dividend_threshold:u8,
        reinvestment_threshold:u8
    ) -> Result<()> {
        
        create_property_system_account::create(
            ctx,
            system_id,
            decimal,
            amount, 
            safety_threshold, 
            trustee_salary_threshold, 
            arbitrator_salary_threshold, 
            dividend_threshold, 
            reinvestment_threshold
        )?;


        Ok(())
    }

//     pub fn create_land_account(
//         ctx:Context<CreateLandAccount>,
//         land_id :u16,
//         legal_doc_hash : [u8;32]
//     )->Result<()>{

//         create_land_acc::create_land_acc(ctx, land_id, legal_doc_hash)?;
//         Ok(())

// }   
    pub fn create_approve_country_authority(
        ctx:Context<CreateApproveCountryAuthority>,
        threshold : u8,
        authority: Vec<Pubkey>,
    )->Result<()>{

        create_approve_country_authority::create_approve_country_authority(ctx, threshold, authority)?;
        Ok(())

    }

    pub fn create_country_proposal(
        ctx:Context<ProposeCountry>,
        country_id: u16,
        country_name: String,
        authority: Vec<Pubkey>,
        country_pda_threshold: u8,
    ) -> Result<()>{
        propose_country_pda::create_country_proposal(ctx, country_id, country_name, authority, country_pda_threshold)?;

    Ok(())
}

    pub fn approve_country(
        ctx:Context<ApproveCountryId>,
    )->Result<()>{

        approve_country_pda::approve_country(ctx)?;

    Ok(())
}

    pub fn execute_country_propsal(
        ctx:Context<ExecuteCountryPda>,
    )->Result<()>{

        execute_country_pda::execute_country_propsal(ctx)?;

        Ok(())

    }

    pub fn create_state_proposal(
    ctx:Context<StateProposal>,
    state_id : u16,
    state_name: String,
    state_authorities : Vec<Pubkey>,
    state_authority_threshold: u8,
)->Result<()>{

    state_proposal::create_state_proposal(ctx, state_id, state_name, state_authorities, state_authority_threshold)?;

    Ok(())
}

    pub fn aprrove_state_proposal(
        ctx:Context<ApproveState>
    )->Result<()>{

        approve_state_proposal::approve(ctx)?;
        
        Ok(())

    }

    pub fn execute_state_proposal(
        ctx:Context<ExecuteStatePda>
    )->Result<()>{

        execute_state_proposal::execute_state_proposal(ctx)?;
    
        Ok(())
    }

    pub fn create_land_proposal(
        ctx:Context<CreateLandProposal>,
        land_id : u64,
        legal_doc_hash: [u8; 32],
    )->Result<()>{

        proposal_land_account::create_proposal(ctx, land_id, legal_doc_hash)?;

        Ok(())

    }

    pub fn approve_land(
        ctx:Context<ApproveLand>
    )->Result<()>{

        approve_land::approve(ctx)?;

        Ok(())

    }
    
    pub fn execute_land(
        ctx:Context<ExecuteLandProposal>
    )->Result<()>{

        execute_land_proposal::execute(ctx)?;

        Ok(())

    }


}
