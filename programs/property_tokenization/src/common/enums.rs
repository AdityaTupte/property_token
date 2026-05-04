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

#[derive(AnchorSerialize, AnchorDeserialize,Clone, Copy, PartialEq, Eq)]
pub enum ProposalType {
    SELLPROPERTY,
    BUYPROPERTY,
    USESAFETY,
    USEREINVESTMENT,
    REVENUETHRESHOLDCHANGE ,
    CHALLLENGEAUTHORITY,
    REMOVEAUTHORITY,
    PROPOSEREMOVEPROPOSAL
}


#[derive(AnchorSerialize, AnchorDeserialize, Clone,Copy, PartialEq, Eq,)]
pub enum AuthorityType {
    TRUSTEE,
    ARBITRATOR 
}



#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq,PartialOrd, Copy,  Eq)]
pub enum ReasonType {
    None,
    Inactivity,
    Other,
    InvalidDocuments,
    DuplicateIdentity,
    Misconduct,
    MaliciousVoting,
    Fraud,
    GovernanceDecision,
}



#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq,PartialOrd, Copy,  Eq)]
pub enum LeaseStatus {
    Active,
    Terminated,
    Expired,
}