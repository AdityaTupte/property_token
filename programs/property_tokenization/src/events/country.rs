use anchor_lang::prelude::*;


#[event]

pub struct AuthorityOfCountryCreated{

    pub country:Pubkey,

    pub authority : Pubkey

}



#[event]

pub struct CountryCreated{

    pub proposal:Pubkey,

    pub country : Pubkey

}




#[event]

pub struct CountryApprovedBySigner{

    pub proposal:Pubkey,

    pub authority : Pubkey

}



#[event]

pub struct AuthorityToApproveCountryCreated{

    pub authority_account : Pubkey

}


#[event]

pub struct CountryProposalCreated{

    pub proposal_key :Pubkey,


}