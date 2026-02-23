use anchor_lang::prelude::*;
pub const PROPERTY_SYSTEM_SEEDS : &[u8] =  b"property_system_account";

pub const LAND_SEED : &[u8] = b"land";

pub const LAND_METADATA_SEEDS : &[u8] = b"metadata";

pub const LAND_PAGE_SEEDS : &[u8] = b"land_page";

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



#[repr(u8)]
pub enum ProposalStatus {
    Active = 1,
    Passed = 2,
    Failed = 3,
}


pub trait Arbitrable {
   
    fn arbitrar_list(&mut self) -> &mut Vec<Pubkey>;

    fn arbitrar_approved(&mut self)-> &mut bool;

    fn proposal_id(& self) -> u64;

  


}