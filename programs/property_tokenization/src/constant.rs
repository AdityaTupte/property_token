use anchor_lang::prelude::*;




p

pub const HARDCODED_PUBKEY: Pubkey = pubkey!("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");

pub const TRANSFERPROPOSAL : &[u8] = b"transferproposal";

pub const VOTERRECIEPT : &[u8] = b"voter_receipt";

pub const BUYPROPERTY : &[u8] = b"BUYPROPERTY";

pub const SELLPROPERTY : &[u8] = b"SELLPROPERTY";

pub const REINVESTMENTPDA : &[u8] = b"reinvestment";

pub const SAFETYPROPOSAL :  &[u8] = b"safetyproposal";

pub const USEREINVESTMENTOKEN :&[u8] = b"use_reinvestment_token";





#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum ProposalType {
    SELLPROPERTY,
    BUYPROPERTY,
    USESAFETY,
    USEREINVESTMENT
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

    fn deadline(&mut self) -> &mut i64;

    fn bump(&mut self) -> &mut u8;
        
}

pub trait Receipt  {

    fn proposal(&mut self) -> &mut Pubkey;

    fn voter(&mut self) -> &mut Pubkey;

    fn voting_power(&mut self) ->&mut u64;

    fn bump(&mut self) -> &mut u8 ;
    
}



