use anchor_lang::prelude::*;



#[event]

pub struct RevenueChangeProposalCreated{

    pub proposal_id:u64,

   pub proposal_key :Pubkey,

   pub property_system : Pubkey,

   pub trustee : Pubkey

}


#[event]
pub struct  SubmitSnapshotForRevenueChangeProposal{

    pub proposal_key :Pubkey,

    pub threshold_submission_deadline_days : i64,
    
    pub voting_for_threshold_deadline : i64,
    
    pub add_new_threshold_deadline : i64,
    
    pub challenge_new_threshold_deadline : i64,

}

#[event]
pub struct ProposedNewThreshold{

    pub proposal : Pubkey,

    pub proposer : Pubkey,

    pub proposed_revenue_threshold : Pubkey,

}


#[event]
pub struct VotedForNewThreshold{

    pub proposal : Pubkey,

    pub new_threshold : Pubkey,

    pub voter : Pubkey,

}

#[event]
pub struct NewThresholdSelected{

    pub proposal : Pubkey,

    pub new_threshold : Pubkey,


}


#[event]
pub struct ChallengeThresholdAccepted{

    pub proposal : Pubkey,

    pub new_threshold : Pubkey,

}


#[event]
pub struct NewThresholdAdopted{

    pub proposal : Pubkey,

    pub new_threshold : Pubkey,

    pub property_system:Pubkey

}