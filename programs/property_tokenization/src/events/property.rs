use anchor_lang::prelude::*;

#[event]

pub struct PropertyProposalCreated{

     pub proposal_key:Pubkey,

     pub created_by:Pubkey,

     pub property_system:Pubkey,
}

#[event]

pub struct VotedForPropertyProposal{

     pub proposal_key:Pubkey,

     pub authority:Pubkey,

}



#[event]

pub struct PropertyCreated{

     pub proposal_key:Pubkey,

     pub property_key :Pubkey,
     pub propoerty_system:Pubkey

}
