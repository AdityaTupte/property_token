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

               //STATECREATION


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
        
        sell_property::sell_proposal_arbitrar_vote(ctx, proposal_id, property_system_id)?;
        Ok(())
    }

    pub fn submit_snapshot_for_sell_proposal(
        ctx:Context<SellSubmitSnapshot>,
        property_system_account:Pubkey,
        proposal_id:u64,
        merkle_root : [u8;32],
        closing_days_gap : u8,
        transfer_deadline_days : u8 ,
        vote_threshold :u64,
    )->Result<()>{

        sell_property::sell_submit_snapshot(ctx, property_system_account, proposal_id, merkle_root, closing_days_gap, transfer_deadline_days, vote_threshold)?;
        
        Ok(())

    }


    pub fn voting_for_sell_proposal(
        ctx:Context<SellProposalVoting>,
        proposal_id:u64,
        property_system_id:u64,
        proof: Vec<[u8; 32]>,
        voting_power : u64,
        yes_or_no : bool,
    )->Result<()>{

            sell_property::vote(ctx, proposal_id, property_system_id, proof, voting_power, yes_or_no)?;

        Ok(())

    }

    pub fn sell_proposal_finalize(
        ctx:Context<SellProposalFinalize>,
        proposal_id:u64,
        property_system_account:Pubkey
    )->Result<()>{

        sell_property::finalize_sell_proposal(ctx, proposal_id, property_system_account)?;

        Ok(())

    }

    pub fn delete_sell(
        ctx:Context<DeleteFailSellProposal>,
        proposal_id:u64,
        property_system_id:u64
    )->Result<()>{

        sell_property::delete_fail_sell_proposal(ctx, proposal_id, property_system_id)?;
        Ok(())
    }



    pub fn create_buy_proposal(
        ctx:Context<BuyPropertyProposal>,
        proposal_id : u64,
        buyer_property_system_id:u64,
        seller_property_system_account:Pubkey,
        seller_proposal_id:u64,
        state_pubkey:Pubkey,
        property_id:u64
    )->Result<()>{

        buy_property_proposal::createbuyproposal(ctx, proposal_id, buyer_property_system_id, seller_property_system_account, seller_proposal_id, state_pubkey, property_id)

    }

    pub fn buy_proposal_arbitrar_vote(
        ctx:Context<ArbitrarVote>,
        proposal_id : u64,
        buyer_property_system_id:u64
    )->Result<()>{

        buy_property::buy_proposal_arbitrar_vote(ctx, proposal_id, buyer_property_system_id)?;
        Ok(())
    }

    pub fn buy_submit_snapshot(
        ctx:Context<BuySubmitSnapshot>,
        property_system_account:Pubkey,
        proposal_id:u64,
        merkle_root : [u8;32],
        closing_days_gap : u8,
        payment_deadline_days : u8 ,
        vote_threshold :u64,
    )->Result<()>{

        buy_property::buy_submit_snapshot(ctx, property_system_account, proposal_id, merkle_root, closing_days_gap, payment_deadline_days, vote_threshold)
    }

    pub fn buy_proposal_voting(
        ctx:Context<BuyProposalVoting>,
        proposal_id:u64,
        property_system_id:u64,
        proof: Vec<[u8; 32]>,
        voting_power : u64,
        yes_or_no : bool,
    )->Result<()>{

        buy_property::vote(ctx, proposal_id, property_system_id, proof, voting_power, yes_or_no)?;
        
        Ok(())
    }

    pub fn buy_proposal_finalize(
        ctx:Context<BuyProposalFinalize>,
        proposal_id:u64,
        property_system_account:Pubkey
    )->Result<()>{

        buy_property::finalize_buy_proposal(ctx, proposal_id, property_system_account)?;
        Ok(())
    }

    pub fn delete_buy_proposal(
        ctx:Context<DeleteFailBuyProposal>,
        proposal_id:u64,
        property_system_id:u64
    )->Result<()>{

        buy_property::delete_buy_proposal(ctx, proposal_id, property_system_id)?;
        Ok(())
    }


    pub fn execute_buy_proposal(
        ctx:Context<ExecuteProposal>,
        proposal_id : u64,
        buyer_property_system_id:u64,
        seller_property_system_account_id:u64,
        seller_proposal_id:u64,
        state_pubkey:Pubkey,
        property_id:u64
    )->Result<()>{

        buy_property::execute_buy_proposal(ctx, proposal_id, buyer_property_system_id, seller_property_system_account_id, seller_proposal_id, state_pubkey,property_id)?;

        Ok(())

    }
    

    // trustee resignation and new trustee election
    
pub fn trustee_resign(ctx:Context<NewTrusteeElectionProposal>, proposal_id:u64, property_system_id:u64)->Result<()>{

    resign_and_elect_new_trustee::new_trustee_election_proposal(ctx, proposal_id, property_system_id)?;

    Ok(())

}

pub fn arbitrar_approve_trustee_election(
        ctx:Context<ArbitrarApproveTrusteeElection>,
        proposal_id:u64,
        property_system_id:u64
)   ->Result<()>{

    resign_and_elect_new_trustee::arbitrar_approve_trustee_election(ctx, proposal_id, property_system_id)?;

    Ok(())


}

pub fn submit_snapshot_for_authority(
    ctx:Context<SubmitSnapshotForAuthority>,

    proposal_id:u64,

    property_system:Pubkey,

    candidate_submission_deadline: u8,

    voting_for_authority_deadline : u8,

    add_new_authority_deadline : u8,

    challenge_new_authority_deadline : u8,

    merkle_root : [u8;32],
)->Result<()>{

    resign_and_elect_new_trustee::submit_snapshot_for_authority(ctx, proposal_id, property_system, candidate_submission_deadline, voting_for_authority_deadline, add_new_authority_deadline, challenge_new_authority_deadline, merkle_root)?;

    Ok(())

}


pub fn create_candidate_profile(
    ctx:Context<CreateCandidateProfile>,
    metadata_hash : [u8;32]
)-> Result<()>{

    create_candidate_profile::create_candidate_profile(ctx, metadata_hash)?;
    Ok(())
}


pub fn submit_trustee_candidate(
    ctx:Context<SubmitTrusteeCandidate>,
    property_system_id:u64,
    proposal_id:u64
)->Result<()>{

    resign_and_elect_new_trustee::submit_trustee_candidate(ctx, property_system_id, proposal_id)?;

    Ok(())
}

pub  fn vote_for_trustee_candiate(
    ctx:Context<VotingForNewTrustee>,
    proposal_id:u64,property_system_id:u64,candidate_key:Pubkey,
    proof: Vec<[u8; 32]>,
    voting_power : u64,
    
)->Result<()>{
    

    resign_and_elect_new_trustee::voting_for_new_trustee(ctx, proposal_id, property_system_id, candidate_key, proof, voting_power)?;

    Ok(())


}


pub fn add_new_trustee(
     ctx:Context<AddNewTrustee>,
    candidate_key:Pubkey,
    property_system_id:u64,
    proposal_id:u64,
    ranking:u8,
)->Result<()>{

    resign_and_elect_new_trustee::add_new_trustee(ctx, candidate_key, property_system_id, proposal_id, ranking)?;

    Ok(())
}


}
