use anchor_lang::prelude::*;
use anchor_spl::associated_token::spl_associated_token_account::solana_program::keccak;
use crate::traits::{Governance, Receipt};
use crate::errors::ErrorCode;
use crate::functions::verify_proof;


pub fn voting<T:Governance , U:Receipt>(
    item:&mut T,
    receipt :&mut U,
    proof: Vec<[u8; 32]>,
    voting_power : u64,
    yes_or_no : bool,
    signer:Pubkey,
    governance_mint:Pubkey,
    proposal_key:Pubkey,
    recepit_bump :u8,
)->Result<()>{

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

     if yes_or_no{

      *item.votes_for() = item
        .votes_for()
        .checked_add(voting_power)
        .ok_or(ErrorCode::MathOverflow)?;

    } 

    else {
        *item.votes_against() = item
        .votes_against()
        .checked_add(voting_power)
        .ok_or(ErrorCode::MathOverflow)?;
    }
   
    *receipt.proposal() = proposal_key;
   
    *receipt.voter() = signer;

    *receipt.voting_power() = voting_power;

    *receipt.bump() = recepit_bump;


    Ok(())
}
