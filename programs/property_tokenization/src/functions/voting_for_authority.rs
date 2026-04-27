use anchor_lang::{ prelude::*};
use anchor_spl::associated_token::spl_associated_token_account::solana_program::keccak;

use crate::{constant::{AuthorityGovernance, BaseProposal}, errors::ErrorCode, functions::verify_proof, state::{AuthorityCandidate, AuthorityVoteReceipt, }};


pub fn voting_for_authority<T:BaseProposal + AuthorityGovernance>(
   proposal_key:Pubkey,
   authority_candidate:&mut AuthorityCandidate,
   receipt:&mut AuthorityVoteReceipt,
   signer:Pubkey,
   item:&mut T,
   proof: Vec<[u8; 32]>,
   receipt_bump : u8,
   voting_power : u64,
   governance_mint :& Pubkey,

)->Result<()>{

   let current_time = Clock::get()?.unix_timestamp;


    require!(
        current_time <= *item.voting_for_authority_deadline() &&
        current_time > *item.candidate_submission_deadline(),
        ErrorCode::AuthorityVotingDeadline
    );

      // require!(authority_candidate.authority_type == AuthorityType::TRUSTEE,ErrorCode::AuthotityTypeNotMatched);

      let leaf = keccak::hashv(&[
        &[*item.proposal_type() as u8],
        signer.as_ref(),
        proposal_key.as_ref(),
        governance_mint.as_ref(),
        &voting_power.to_le_bytes(),
    ]).0;
      

      require!(
        verify_proof(leaf, &proof, *item.merkle_root()),
        ErrorCode::InvalidMerkleProof
    );
        
        receipt.voter = signer;

    //change here later
        receipt.candidate_pubkey = authority_candidate.candidate;

        receipt.proposal = proposal_key;

        receipt.voting_power = voting_power;

        receipt.bump = receipt_bump;

    

    // require!(item.authority_to_resign().len() > receipt.votes.len(),ErrorCode::VotingLimitReached);

   // require!(!receipt.votes.contains(&authority_candidate.candidate),ErrorCode::DuplicateAuthority);

    // authority_candidate.vote_gained = authority_candidate.vote_gained
    //                             .checked_add(voting_power)
    //                             .ok_or(ErrorCode::MathOverflow)?;
    

    // receipt.votes.push(authority_candidate.candidate);

    authority_candidate.vote_gained = authority_candidate.vote_gained
                                            .checked_add(voting_power)
                                            .ok_or(ErrorCode::MathOverflow)?;


   Ok(())
}
   

