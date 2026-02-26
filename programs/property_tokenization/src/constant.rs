use anchor_lang::prelude::*;
pub const PROPERTY_SYSTEM_SEEDS : &[u8] =  b"property_system_account";

pub const PROPERTY_SEED : &[u8] = b"property";

pub const PROPERTY_METADATA_SEEDS : &[u8] = b"metadata";

pub const PROPERTY_PAGE_SEEDS : &[u8] = b"property_page";

pub const HARDCODED_PUBKEY: Pubkey = pubkey!("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");

pub const TRANSFERPROPOSAL : &[u8] = b"transferproposal";

pub const VOTERRECIEPT : &[u8] = b"voter_receipt";

pub const BUYPROPOSAL : &[u8] = b"buyproposal";

pub const SELLPROPERTY : &[u8] = b"SELLPROPERTY";


#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum ProposalType {
    SELLPROPERTY,
    Buy,
}

pub const MAX_LAND_PER_PAGE:usize = 100;



#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum ProposalStatus {
    Draft,
    Active ,
    Passed ,
    Failed,
    Rejected,
    Executed ,
}


pub trait Governance {

    fn proposal_id(&mut self) -> &mut u64;

    fn start_time(&mut self) -> &mut i64; 

    fn end_time(&mut self) -> &mut i64;

    fn merkle_root(&mut self) -> &mut [u8;32] ;

    fn arbitrar_list(&mut self) -> &mut Vec<Pubkey>;

    fn arbitrar_approved(&mut self)-> &mut bool;

    fn total_voting_power(&mut self) -> &mut u64;

    fn vote_threshold(&mut self) -> &mut u64;

    fn votes_for(&mut self) -> &mut u64;

    fn votes_against(&mut self) -> &mut u64;

    fn proposal_status(&mut self) -> &mut ProposalStatus;
    
    fn snapshot_submitted(&mut self) -> &mut bool;

    fn proposal_type(&mut self) -> &mut ProposalType;

    fn slot(&mut self) -> &mut u64;

    fn bump(&mut self) -> &mut u8;
        
}

// pub trait Arbitrable {
   
//     fn arbitrar_list(&mut self) -> &mut Vec<Pubkey>;

//     fn arbitrar_approved(&mut self)-> &mut bool;

//     fn proposal_id(& self) -> u64;

// }


// pub trait Snapshot {

//     fn merkle_root(&mut self) -> &mut [u8;32] ;

//     fn start_time(&mut self) -> &mut i64;

//     fn end_time(&mut self) -> &mut i64;

//     fn snapshot_submitted(&mut self) -> &mut bool;

//     fn total_voting_power(& self) ->  u64;

//     fn vote_required(&mut self) -> &mut u64;

//     fn proposal_status(&mut self) -> &mut ProposalStatus;

// }

// pub trait Vote {

//     fn merkle_root(&mut self) -> &mut [u8;32] ;

//     fn start_time(&mut self) -> &mut i64;

//     fn end_time(&mut self) -> &mut i64;

//     fn total_voting_power(& self) ->  u64;

//     fn votes_for(&mut self) -> &mut u64;

//     fn votes_against(&mut self) -> &mut u64;

//     fn vote_required(&mut self) ->  u64;


// }



pub trait Receipt  {

    fn proposal(&mut self) -> &mut Pubkey;

    fn voter(&mut self) -> &mut Pubkey;

    fn voting_power(&mut self) ->&mut u64;

    fn bump(&mut self) -> &mut u8 ;
    
}
