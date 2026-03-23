use anchor_lang::prelude::*;
pub const MAX_TRUSTEES: usize = 5 ;
pub const MAX_ARBITRATOR: usize = 5 ;


pub const COUNTRY_APPROVE_AUTHORITY_SEEDS: &[u8] = b"AuthorityForApprovingCountry";
pub const MAX_COUNTRY_APPROVE_AUTHORITY :usize = 10;


pub const COUNTRY_PROPOSAL_SEEDS: &[u8] = b"country_proposal";
pub const  COUNTRY_SEED : &[u8] = b"country";


pub const MAX_COUNTRY_NAME: usize = 64;
pub const MAX_COUNTRY_AUTHORITY: usize = 10;
    


pub const STATE_PROPOSAL_SEEDS: &[u8] = b"state_proposal";
pub const STATE_SEEDS : &[u8] = b"state";

pub const MAX_STATE_NAME: usize = 50;
pub const MAX_STATE_AUTHORITIES: usize = 10;


pub const PROPERTY_SYSTEM_SEEDS : &[u8] =  b"property_system_account";
pub const PROPERTY_PAGE_SEEDS : &[u8] = b"property_page";



pub const PROPERTY_PROPOSAL_SEEDS: &[u8] = b"property_proposal";
pub const PROPERTY_SEED : &[u8] = b"property";
pub const PROPERTY_METADATA_SEEDS :&[u8] = b"property_metadata";



pub const SELLPROPERTY : &[u8] = b"SELLPROPERTY";
pub const BUYPROPERTY : &[u8] = b"BUYPROPERTY";

pub const REINVESTMENTPDA : &[u8] = b"reinvestment";
pub const SAFETYPDA :  &[u8] = b"safety";

pub const SAFETYPROPOSAL :  &[u8] = b"safetyproposal";

pub const USEREINVESTMENTOKEN :&[u8] = b"use_reinvestment_token";


pub const TRUSTEEREGISTRYSEEDS : &[u8] = b"trustee_registry";
pub const ARBITRAR_REGISTRYSEEDS : &[u8] = b"arbitrar_registry";

pub const TREASURYSEEDS : &[u8] = b"treasury";



pub const HARDCODED_PUBKEY: Pubkey = pubkey!("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");

pub const TRANSFERPROPOSAL : &[u8] = b"transferproposal";

pub const VOTERRECIEPT : &[u8] = b"voter_receipt";

pub const MAX_LAND_PER_PAGE:usize = 100;



pub const RT_CHG_PROPOSAL_SEEDS : &[u8] = b"rt_change_proposal";
pub const PROPOSE_THRESHOLD : &[u8] = b"propose_threshold";

pub const THRESHOLD_VOTE_RECEIPT : &[u8] = b"threshold_vote_receipt";

pub const THRESHOLD :&[u8] = b"threshold";


pub const TRUSTEE_RESIGNATION : &[u8] = b"trustee_resignation";

pub const ARBITRAR_RESIGNATION : &[u8] = b"arbitrar_resignation";


pub const ELECT_TRUSTEE : &[u8] = b"elect_trustee";

pub const ELECT_ARBITRAR : &[u8] = b"elect_arbitrar" ;


pub const CANDIDATE_PROFILE : &[u8] = b"candidate_profile";

pub const AUTHORITY_CANDIDATE : &[u8] = b"authority_candidate";

pub const AUTHORITYVOTERECEIPT : &[u8] = b"authority_vote_receipt";

