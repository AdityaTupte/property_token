use anchor_lang::prelude::*;



#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum ProposalStatus {
    Draft,
    Active ,
    Passed ,
    Failed,
    Rejected,
    Executed,
    Pending,
    Approved,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum ProposalType {
    SELLPROPERTY,
    BUYPROPERTY,
    USESAFETY,
    USEREINVESTMENT,
    REVENUETHRESHOLDCHANGE  
}


#[derive(AnchorSerialize, AnchorDeserialize, Clone,Copy, PartialEq, Eq)]
pub enum AuthorityType {
    TRUSTEE,
    ARBITRATOR 
}



#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum ReasonType {
    None,

    // governance related
    Fraud,
    Misconduct,
    Inactivity,
    MaliciousVoting,

    // system-level
    DuplicateIdentity,
    InvalidDocuments,

    // manual/admin
    GovernanceDecision,

    Other,
}
