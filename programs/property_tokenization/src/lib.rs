use anchor_lang::prelude::*;
pub mod instructions;
pub mod state;
pub mod events;
pub mod errors;
pub mod traits;
pub mod common;
pub mod functions;
use crate::instructions::*;
use crate::common::*;

declare_id!("BD4CURALcvWKHuLrcx74iaJaaUrKbRogXYXcNBPfqFRR");

#[program]
pub mod property_tokenization {

    use crate::common::ReasonType;

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


pub fn adjust_ranks(
    ctx:Context<AdjustTrusteeRanking>,
    proposal_id:u64,
    property_system:Pubkey,
    candidate_key1:Pubkey,
    candidate_key2:Pubkey,
    ranking1:u8,
    ranking2:u8
)->Result<()>{


    resign_and_elect_new_trustee::adjust_trustee_ranking(ctx, proposal_id, property_system, candidate_key1, candidate_key2,ranking1,ranking2)?;

    Ok(())

}

pub fn challenge_against_new_trustee(
    ctx:Context<ChallengeNewTrustee>,
    proposal_id:u64,
    challenge_from_key:Pubkey,
    challenge_to_key:Pubkey,
    ranking:u8,
    property_system_id:u64
)->Result<()>{
    resign_and_elect_new_trustee::challenge_new_trustee(ctx, proposal_id, challenge_from_key, challenge_to_key, ranking,property_system_id)?;
    Ok(())
}


pub fn  finalize_old_trsutee(
    ctx:Context<FinalizeTrustee>,
    proposal_id:u64,
    property_system_id:u64,
    trustee:Pubkey
)->Result<()>{

    resign_and_elect_new_trustee::finalize_trustee(ctx, proposal_id, property_system_id, trustee)?;
    
    Ok(())

}


pub fn finalize_new_trustee(
    ctx:Context<FinalizeTrusteeAuthorityCandiate>,
    candidate_pubkey:Pubkey,proposal_id:u64,property_system_id:u64
)
->Result<()>{

    resign_and_elect_new_trustee::finalize_trustee_authority_candiate(ctx, candidate_pubkey, proposal_id, property_system_id)?;

    Ok(())

}


    //Arbitar resgination and election


pub fn arbitrar_resign(ctx:Context<NewArbitrarElectionProposal>, proposal_id:u64, property_system_id:u64)->Result<()>{

    resign_and_elect_new_arbitrar::new_arbitrar_election_proposal(ctx, proposal_id, property_system_id)?;

    Ok(())


}

pub fn trustee_approve_arbitrar_election(
        ctx:Context<TrusteeApproveArbitrarElection>,
        proposal_id:u64,
        property_system_id:u64
)   ->Result<()>{

    resign_and_elect_new_arbitrar::trustee_approve_arbitrar_election(ctx, proposal_id, property_system_id)?;

    Ok(())


}


pub fn submit_snapshot_for_arbitrar_election(
    ctx:Context<SubmitSnapshotForArbitrarAuthority>,
    proposal_id:u64,

    property_system:Pubkey,

    candidate_submission_deadline: u8,

    voting_for_authority_deadline : u8,

    add_new_authority_deadline : u8,

    challenge_new_authority_deadline : u8,

    merkle_root : [u8;32],
)->Result<()>{

    resign_and_elect_new_arbitrar::submit_snapshot_for_arbitrar_authority(ctx, proposal_id, property_system, candidate_submission_deadline, voting_for_authority_deadline, add_new_authority_deadline, challenge_new_authority_deadline, merkle_root)?;
    Ok(())

}

pub fn submit_arbitrar_candidate(
    ctx:Context<SubmitArbitrarCandidate>,
    property_system_id : u64,
    proposal_id : u64
)->Result<()>{

    resign_and_elect_new_arbitrar::submit_arbitrar_candidate(ctx, property_system_id, proposal_id)?;

    Ok(())

}


pub  fn vote_for_arbitrar_candiate(
    ctx:Context<VotingForNewArbitrar>,
    proposal_id:u64,property_system_id:u64,candidate_key:Pubkey,
    proof: Vec<[u8; 32]>,
    voting_power : u64,
    
)->Result<()>{
    

    resign_and_elect_new_arbitrar::voting_for_new_arbitrar(ctx, proposal_id, property_system_id, candidate_key, proof, voting_power)?;

    Ok(())


}


pub fn add_new_arbitrar(
    ctx:Context<AddNewArbitrar>,
    candidate_key:Pubkey,
    property_system_id:u64,
    proposal_id:u64,
    ranking:u8,
)-> Result<()> {

resign_and_elect_new_arbitrar::add_new_arbitrar(ctx, candidate_key, property_system_id, proposal_id, ranking)?;

Ok(())

}



pub fn adjust_arbitrar_ranks(
    ctx:Context<AdjustArbitrarRanking>,
    proposal_id:u64,
    property_system:Pubkey,
    candidate_key1:Pubkey,
    candidate_key2:Pubkey,
    ranking1:u8,
    ranking2:u8
)->Result<()>{


    resign_and_elect_new_arbitrar::adjust_arbitrar_ranking(ctx, proposal_id, property_system, candidate_key1, candidate_key2, ranking1, ranking2)?;

    Ok(())

}


pub fn challenge_against_new_arbitrar(
     ctx:Context<ChallengeNewArbitrar>,
    proposal_id:u64,
    challenge_from_key:Pubkey,
    challenge_to_key:Pubkey,
    ranking:u8,
    property_system_id:u64,
)->Result<()>{

    resign_and_elect_new_arbitrar::challenge_new_arbitrar(ctx, proposal_id, challenge_from_key, challenge_to_key, ranking, property_system_id)?;

    Ok(())
}



pub fn  finalize_old_arbitrar(
    ctx:Context<FinalizeOldArbitrar>,
    proposal_id:u64,
    property_system_id:u64,
    arbitrar:Pubkey
)->Result<()>{

    resign_and_elect_new_arbitrar::finalize_old_arbitrar(ctx, proposal_id, property_system_id, arbitrar)?;
    
    Ok(())

}


pub fn finalize_new_arbitrar(
        ctx:Context<FinalizeArbitrarAuthorityCandiate>,
        candidate_pubkey:Pubkey,
        proposal_id:u64,
        property_system_id:u64

)->Result<()>{


    resign_and_elect_new_arbitrar::finalize_arbitrar_authority_candiate(ctx, candidate_pubkey, proposal_id, property_system_id)?;

    Ok(())

}


pub fn challenge_authority(
    ctx:Context<ChallengeAuthorityProposal>,
    proposal_id : u64,
    property_system_id : u64,
    charges_hash: [u8; 32],
    evidence_hash : [u8;32],
)->Result<()>{

    challenge_against_authority::challenge_authority(ctx, proposal_id, property_system_id, charges_hash, evidence_hash)?;

    Ok(())


}

pub fn add_trustee_offender(
    ctx:Context<AddTrusteeOffender>,
    proposal_id : u64,property_system_id : u64
)->Result<()>{

    challenge_against_authority::add_trustee_offender(ctx, proposal_id, property_system_id)?;

    Ok(())

}


pub fn add_arbitrar_offender(
    ctx:Context<AddArbitrarOffender>,
    proposal_id : u64,property_system_id : u64
)->Result<()>{

    challenge_against_authority::add_arbitrar_offender(ctx, proposal_id, property_system_id)?;

    Ok(())

}
   


pub fn submit_snaphot_for_voting_on_challenge_proposal(
     ctx: Context<SubmitSnaphotForChallengeProposal>,
    proposal_id : u64,property_system_id : u64,merkle_root : [u8;32],
)->Result<()>{

    challenge_against_authority::submit_snapshot_for_challenge_proposal(ctx, proposal_id, property_system_id, merkle_root)?;

    Ok(())
}


pub fn vote_for_challenge_proposal(
    ctx:Context<VoteForChallengeProposal>,
    proposal_id : u64,
    property_system_id : u64,
    proof: Vec<[u8;32]>,
    voting_power : u64,
)->Result<()>{

    challenge_against_authority::vote_for_challenge_proposal(ctx, proposal_id, property_system_id, proof, voting_power)?;
    Ok(())
}


pub fn outcome_of_proposal(
    ctx:Context<OutComeOFProposal>,
    proposal_id : u64,
    property_system_id : u64,
    outcome:ReasonType,
)->Result<()>{
    
    challenge_against_authority::outcome_of_proposal(ctx, proposal_id, property_system_id, outcome)?;

    Ok(())

}


pub fn finalize_trustee_candidate_profile_for_challenge_proposal(
            ctx:Context<FinalizeTrusteeCandidateProfile>,
            proposal_id:u64,
            property_system_id:u64,
            candidate_key:Pubkey
)->Result<()>{

    challenge_against_authority::finalize_trustee_candidate_profile(ctx, proposal_id, property_system_id, candidate_key)?;
        Ok(())
}

pub fn finalize_arbitrar_candidate_profile_for_challenge_proposal(
            ctx:Context<FinalizeArbitrarCandidateProfile>,
            proposal_id:u64,
            property_system_id:u64,
            candidate_key:Pubkey
)->Result<()>{

    challenge_against_authority::finalize_arbitrar_candidate_profile(ctx, proposal_id, property_system_id, candidate_key)?;
        Ok(())
}


pub fn remove_trustee_guilt_authority_proposal(
    ctx:Context<RemoveGuiltyTrusteeAuthority>,
    proposal_id:u64,
    property_system_id:u64
)->Result<()>{


    challenge_against_authority::removal_of_trustee_proposal(ctx, proposal_id, property_system_id)?;

    Ok(())
}

pub fn remove_arbitrar_guilt_authority_proposal(
    ctx:Context<RemoveGuiltyArbitrarAuthority>,
    proposal_id:u64,
    property_system_id:u64
)->Result<()>{


    challenge_against_authority::removal_of_arbitrar_proposal(ctx, proposal_id, property_system_id)?;

    Ok(())

}



pub fn add_trustee_for_removal(
    ctx:Context<AddTrusteeToRemove>,
    proposal_id:u64,
    property_system_id:u64,
  
)->Result<()>{

    challenge_against_authority::add_trustee_for_removal_proposal(ctx, proposal_id, property_system_id)?;

    Ok(())
}

pub fn add_arbitrar_for_removal(
    ctx:Context<AddArbitrarToRemove>,
    proposal_id:u64,
    property_system_id:u64,
    
)->Result<()>{

    challenge_against_authority::add_arbitrar_for_removal_proposal(ctx, proposal_id, property_system_id)?;

    Ok(())
}

pub fn ask_snapshot_for_remove_proposal(

    ctx:Context<AskForSnapshotOFRemoveProposal>

)->Result<()>{

    challenge_against_authority::ask_snapshot_for_remove_proposal(ctx)?;

    Ok(())

}



pub fn submit_snapshot_for_removal_proposal(
    ctx:Context<SubmitSnapshotForGuiltyAuthority>,
    challenge_proposal_key:Pubkey,property_system_id:u64,
    merkle_root : [u8;32],
)->Result<()>{

    challenge_against_authority::submit_snapshot_for_removal_of_guilty_authority(ctx, challenge_proposal_key, property_system_id, merkle_root)?;

    Ok(())
}

pub fn submit_candidate_for_trustee_authority_for_remove_proposal(
    ctx:Context<SubmitCandidateForTrusteeAuthority>,
    proposal_key:Pubkey,
    property_system_id:u64
)->Result<()>{

    challenge_against_authority::submit_candidate_for_trustee_authority(ctx, proposal_key, property_system_id)?;

    Ok(())
}

pub fn submit_candidate_for_arbitrar_authority_for_remove_proposal(
    ctx:Context<SubmitCandidateForArbitrarAuthority>,
    proposal_key:Pubkey,
    property_system_id:u64
)->Result<()>{

    challenge_against_authority::submit_candidate_for_arbitrar_authority(ctx, proposal_key, property_system_id)?;

    Ok(())
}



pub fn vote_for_new_trustee_authority_for_removal_proposal(
    ctx:Context<VoteForNewTrusteeAuthority>,
    proposal_key:Pubkey,property_system_id:u64,
    candidate_key:Pubkey,
    proof: Vec<[u8; 32]>,
    voting_power : u64,
)->Result<()>{

    challenge_against_authority::vote_for_new_trustee_authority(ctx, proposal_key, property_system_id,candidate_key, proof, voting_power)?;

    Ok(())
}

pub fn vote_for_new_arbitrar_authority_for_removal_proposal(
    ctx:Context<VoteForNewArbitrarAuthority>,
    proposal_key:Pubkey,property_system_id:u64,
    candidate_key:Pubkey,
    proof: Vec<[u8; 32]>,
    voting_power : u64,
)->Result<()>{

    challenge_against_authority::vote_for_new_arbitrar_authority(ctx, proposal_key, property_system_id,candidate_key, proof, voting_power)?;

    Ok(())
}


pub fn finalize_remove_proposal(
        ctx:Context<FinalizeRemoveAuthority>
)->Result<()>{

    challenge_against_authority::finalize_remove_proposal(ctx)?;

    Ok(())

}

pub fn add_new_authority_for_trustee_remove_proposal(
     ctx:Context<AddNewTrusteeAuthority>,
     proposal_key:Pubkey,
    candidate_key:Pubkey,
    property_system_id:u64,
    proposal_id:u64,
    ranking:u8,
)->Result<()>{

    challenge_against_authority::add_new_authority_for_remove_proposal(ctx,proposal_key, candidate_key, property_system_id, proposal_id, ranking)?;

    Ok(())
}

pub fn add_new_authority_for_arbitrar_remove_proposal(
     ctx:Context<AddNewArbitrarAuthority>,
     proposal_key:Pubkey,
    candidate_key:Pubkey,
    property_system_id:u64,
    proposal_id:u64,
    ranking:u8,
)->Result<()>{

    challenge_against_authority::add_new_arbitrar_authority_for_remove_proposal(ctx,proposal_key, candidate_key, property_system_id, proposal_id, ranking)?;

    Ok(())
}


pub fn adjust_ranking_of_new_authority_for_remove_proposal(
    ctx:Context<AdjustAuthorityRanking>,
    proposal_key:Pubkey,
    property_system:Pubkey,
    candidate_key1:Pubkey,
    candidate_key2:Pubkey,
    ranking1:u8,
    ranking2:u8
)->Result<()>{

    challenge_against_authority::adjust_arbitrar_ranking(ctx,proposal_key, property_system, candidate_key1, candidate_key2, ranking1, ranking2)?;

    Ok(())
}   


pub fn challenge_new_authority_of_removal_prposal(
    ctx:Context<RemovalProposalChallengeNewAuthority>,
    proposal_key:Pubkey,property_system_id:u64,challenge_from_key:Pubkey,challenge_to_key:Pubkey,ranking:u8
)->Result<()>{

    challenge_against_authority::challenge_new_authority(ctx, proposal_key, property_system_id, challenge_from_key, challenge_to_key, ranking)?;

    Ok(())
}

pub fn remove_old_trustee_remove_proposal(
    ctx:Context<RemoveOldTrustee>,
    proposal_id:u64,
    proposal_key:Pubkey,
    property_system_id:u64,
    trustee:Pubkey
)->Result<()>{
    
    challenge_against_authority::remove_old_trustee(ctx, proposal_id, proposal_key, property_system_id, trustee)?;

    Ok(())
}

pub fn remove_old_arbitrar_remove_proposal(
    ctx:Context<RemoveOldArbitrar>,
    proposal_id:u64,
    proposal_key:Pubkey,
    property_system_id:u64,
    arbitrar:Pubkey
)->Result<()>{
    
    challenge_against_authority::remove_old_arbitrar(ctx, proposal_id, proposal_key, property_system_id, arbitrar)?;

    Ok(())
}

pub fn finalize_new_trustee_for_remove_proposal(
    ctx:Context<FinalizeNewTrustee>,
     candidate_pubkey:Pubkey,proposal_id:u64,property_system_id:u64,proposal_key:Pubkey,
)->Result<()>{

    challenge_against_authority::finalize_new_trustee(ctx, candidate_pubkey, proposal_id, property_system_id, proposal_key)?;

    Ok(())
}


pub fn finalize_new_arbitrar_for_remove_proposal(
    ctx:Context<FinalizeNewArbitrar>,
     candidate_pubkey:Pubkey,proposal_id:u64,property_system_id:u64,proposal_key:Pubkey,
)->Result<()>{

    challenge_against_authority::finalize_new_arbitrar(ctx, candidate_pubkey, proposal_id, property_system_id, proposal_key)?;

    Ok(())
}

 //////
 


pub fn initialize_lease_proposal(
    ctx:Context<InitializeLeaseProposal>,
    lease_id :u64,
    property_id:u64,
    state_pubkey:Pubkey,
    property_system_id:u64,
    rent : u64,
    security_deposit:u64,
    agreement_hash :[u8;32],
    end_time_in_days : u32,
    late_payment_fee_per_day : u64,
    periodic_pay : i64,
)->Result<()>{

    lease_property::initialize_lease_proposal(ctx, lease_id, property_id, state_pubkey,property_system_id, rent, security_deposit, agreement_hash, end_time_in_days, late_payment_fee_per_day, periodic_pay)?;

    Ok(())

}


pub fn arbitrar_approval_for_lease(
    ctx:Context<ArbitrarApprovalForLease>,
    lease_id:u64,property:Pubkey,property_system_id:u64
)->Result<()>{

    lease_property::arbitrar_approval_for_lease(ctx, lease_id, property,property_system_id)?;

    Ok(())
}


pub fn lease_accept(
        ctx:Context<LesseeAcceptance>,
    property_system_id:u64,lease_id:u64
)->Result<()>{

    lease_property::lessee_acceptance(ctx,  property_system_id, lease_id)?;

    Ok(())

}


pub fn pay_rent(
        ctx:Context<PayRent>,
    property_system:Pubkey,lease_id:u64,lease_property:Pubkey
)->Result<()>{

    lease_property::pay_rent(ctx, property_system, lease_id, lease_property)?;

    Ok(())

}


///// token transfer  safety




pub fn token_transfer_create_use_safety_proposal(
    ctx:Context<UseSafetyTokensProposal>,
    proposal_id:u64,
    property_system_id:u64,
    amount_required:u64,
    reason_hash:[u8;32],
)->Result<()>{

    token_transfer_proposal::create_use_safety_proposal(ctx,proposal_id, property_system_id,  amount_required, reason_hash)?;

    Ok(())

}



pub fn token_transfer_arbitrar_approval_safety_proposal(
     ctx:Context<SafetyArbitrarVote>,
    proposal_id : u64,property_system_id:u64
)->Result<()>{

token_transfer_proposal::arbitrar_vote(ctx, proposal_id, property_system_id)?;


Ok(())
}


pub fn token_transfer_submit_snapshot_safety_proposal(
    ctx:Context<SubmitSnapshot>,
    property_system_account:Pubkey,proposal_id:u64,
    merkle_root : [u8;32],
    closing_days_gap : u8,
    deadline_days : u8 ,
    vote_threshold :u64,
)->Result<()>{

token_transfer_proposal::saftey_submit_snapshot(ctx, property_system_account, proposal_id, merkle_root, closing_days_gap, deadline_days, vote_threshold)?;


            Ok(())
}

pub fn token_transfer_vote_for_submit_proposal(
ctx:Context<Voting>,
        proposal_id:u64,property_system_id:u64,
        proof: Vec<[u8; 32]>,
        voting_power : u64,
        yes_or_no : bool,
    )->Result<()>{
    
        token_transfer_proposal::vote(ctx, proposal_id, property_system_id, proof, voting_power, yes_or_no)?;

    Ok(())

}


pub fn token_transfer_finalize_safety_proposal(
         ctx:Context<Finalize>,
         proposal_id:u64,property_system:Pubkey
)->Result<()>{


    token_transfer_proposal::finalize_safety_proposal(ctx, proposal_id, property_system)?;


    Ok(())

}


pub fn token_transfer_execute_safety_proposal(
          ctx:Context<ExecuteSafety>,
    proposal_id:u64,property_system_id:u64
)->Result<()>{


    token_transfer_proposal::execute_safety_proposal(ctx, proposal_id, property_system_id)?;


    Ok(())

}

pub fn token_transfer_delete_safety_proposal(
          ctx:Context<DeleteSafetyFailProposal>,
    proposal_id:u64,property_system_id:u64
)->Result<()>{


    token_transfer_proposal::delete_buy_safety_proposal(ctx, proposal_id, property_system_id)?;


    Ok(())

}

//token transfer reinvestment



pub fn token_transfer_create_use_reinvest_proposal(
    ctx:Context<UseReinvestmentTokensProposal>,
    proposal_id:u64,
    property_system_id:u64,
    amount_required:u64,
    reason_hash:[u8;32],
)->Result<()>{

    token_transfer_proposal::create_use_reinvest_proposal(ctx,proposal_id, property_system_id,  amount_required, reason_hash)?;

    Ok(())

}



pub fn token_transfer_arbitrar_approval_reivest_proposal(
     ctx:Context<ReinvestArbitrarVote>,
    proposal_id : u64,property_system_id:u64
)->Result<()>{

token_transfer_proposal::arbitrar_vote_for_reivest(ctx, proposal_id, property_system_id)?;


Ok(())
}


pub fn token_transfer_submit_snapshot_reinvest_proposal(
    ctx:Context<ReinvestSubmitSnapshot>,
    property_system_account:Pubkey,proposal_id:u64,
    merkle_root : [u8;32],
    closing_days_gap : u8,
    deadline_days : u8 ,
    vote_threshold :u64,
)->Result<()>{

token_transfer_proposal::reinvest_submit_snapshot(ctx, property_system_account, proposal_id, merkle_root, closing_days_gap, deadline_days, vote_threshold)?;


            Ok(())
}

pub fn token_transfer_vote_for_reinvest_proposal(
ctx:Context<ReinvestVoting>,
        proposal_id:u64,property_system_id:u64,
        proof: Vec<[u8; 32]>,
        voting_power : u64,
        yes_or_no : bool,
    )->Result<()>{
    
        token_transfer_proposal::reivestment_vote(ctx, proposal_id, property_system_id, proof, voting_power, yes_or_no)?;

    Ok(())

}


pub fn token_transfer_finalize_reivest_proposal(
         ctx:Context<ReinvestFinalize>,
         proposal_id:u64,property_system:Pubkey
)->Result<()>{


    token_transfer_proposal::finalize_reinvest_proposal(ctx, proposal_id, property_system)?;


    Ok(())

}


pub fn token_transfer_execute_reinvest_proposal(
          ctx:Context<ExecuteReinvestment>,
    proposal_id:u64,property_system_id:u64
)->Result<()>{


    token_transfer_proposal::execute_reivestment_proposal(ctx, proposal_id, property_system_id)?;


    Ok(())

}

pub fn token_transfer_delete_reinvest_proposal(
          ctx:Context<DeleteFailReinvestProposal>,
    proposal_id:u64,property_system_id:u64
)->Result<()>{


    token_transfer_proposal::delete_reinvest_proposal(ctx, proposal_id, property_system_id)?;


    Ok(())

}






}
