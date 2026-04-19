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
        total_trustees:u8,
        trustee_vote_threshold:u8,
        total_arbitrar:u8,
        arbitrar_vote_threshold:u8,
    )->Result<()>{

        property_system::create(ctx, system_id, decimal, number_of_tokens, safety_threshold, trustee_salary_threshold, arbitrator_salary_threshold, dividend_threshold, reinvestment_threshold, total_trustees, trustee_vote_threshold, total_arbitrar, arbitrar_vote_threshold)?;


        Ok(())
    }
   
                    //ADD TRUSTEE

    pub fn add_trustee(ctx: Context<AddTrustee>, system_id:u64) -> Result<()> {

        property_system::add_trustee(ctx, system_id)?;

        Ok(())
    }




                    //ADD ARBITRATOR


    pub fn add_arbitrator(ctx: Context<AddArbitrator>, system_id:u64) -> Result<()> {

        property_system::add_arbitrator(ctx, system_id)?;

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
    state_total_authorities :u8,
    state_authority_threshold: u8,
    )->Result<()>{

        state_creation::create_state_proposal(ctx, state_name, country_name, state_id, state_total_authorities, state_authority_threshold)?;

        Ok(())


    }

    pub fn state_proposal_approval(
        ctx:Context<ApproveState>,
         state_name:[u8;32],
    country_name:[u8;32]
    )->Result<()>{

        state_creation::approve_state(ctx, state_name, country_name)?;

        Ok(())
    }


    pub fn state_proposal_execute(
        ctx:Context<ExecuteStatePda>,
         state_name:[u8;32],
    country_name:[u8;32]
    )->Result<()>{
        state_creation::execute_state_proposal(ctx, state_name, country_name)?;

        Ok(())
    }


    pub fn add_state_auhtority(
         ctx:Context<AddStateAuthority>,
         country_name:[u8;32],
          state_name:[u8;32],
    )->Result<()>{


        state_creation::add_state_authority(ctx, country_name, state_name)?;

            Ok(())
    }

    pub fn create_property_proposal(
        ctx:Context<CreatePropertyProposal>,
        country_key:Pubkey,
        state_name:[u8;32],
        property_id : u64,
        property_system_id:u64,
        legal_doc_hash: [u8; 32],
    )->Result<()>{
        property_creation::create_proposal(ctx, country_key, state_name, property_id, property_system_id, legal_doc_hash)?;
            Ok(())
    }

    pub fn approve_property_proposal(
        ctx:Context<ApproveLand>,
        country_key:Pubkey,state_name:[u8;32],property_id:u64,
    )->Result<()>{

        property_creation::approve(ctx, country_key, state_name, property_id)?;
            Ok(())

    }

    pub fn execute_property_proposal(
        ctx:Context<ExecutePropertyProposal>,
        country_key:Pubkey,state_name:[u8;32],property_id:u64,property_system_id:u64,
    )->Result<()>{

        property_creation::execute(ctx, country_key, state_name, property_id, property_system_id,)?;
        Ok(())

    }


            // SELL PROPERTY

    pub fn create_sell_proposal(
        ctx:Context<SellLandProposal>,
        proposal_id: u64,
        property_id:u64,
        property_system_id:u64,
        state_pubkey:Pubkey,
        sale_price:u64
    )->Result<()>{

        sell_property_proposal::create_sell_proposal(ctx, proposal_id, property_id, property_system_id, state_pubkey, sale_price)?;
        Ok(())
    }


    pub fn sell_proposal_arbitrar_vote(ctx:Context<ArbitrarApproval>,proposal_id: u64,property_system_id:u64 )->Result<()>{
        
        sell_proposal_arbitrar_vote::sell_proposal_arbitrar_vote(ctx, proposal_id, property_system_id)?;
        Ok(())
    }

}
