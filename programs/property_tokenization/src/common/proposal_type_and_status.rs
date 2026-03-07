use anchor_lang::prelude::*;



#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum ProposalStatus {
    Draft,
    Active ,
    Passed ,
    Failed,
    Rejected,
    Executed ,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum ProposalType {
    SELLPROPERTY,
    BUYPROPERTY,
    USESAFETY,
    USEREINVESTMENT
}
