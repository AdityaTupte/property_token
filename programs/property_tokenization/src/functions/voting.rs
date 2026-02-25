use anchor_lang::prelude::*;
use anchor_spl::associated_token::spl_associated_token_account::solana_program::keccak;
use crate::errors::ErrorCode;
use crate::functions::verify_proof;



pub fn voting_check<T:Vote,U:Snapshot> (
    item: &T,
    iter : &U,
    proof: Vec<[u8;32]>,
    voting_power: u64,
    yes_or_no : bool,
    signer:Pubkey,
    governance_mint:Pubkey,
)->Result<()>{

    let current_time = Clock::get()?.unix_timestamp;
    
    require!(current_time >= ite.start_time  && current_time <= iter.end_time , ErrorCode::VotingPeriodExpired);

     require!(voting_power <= iter.total_voting_power(), ErrorCode::VotingPowerInvalid);

    let leaf = keccak::hashv(&[
        signer.as_ref(),
        governance_mint.as_ref(),
        &voting_power.to_le_bytes(),
    ]).0;

    
    require!(
        verify_proof(leaf, &proof, iter.merkle_root),
        ErrorCode::InvalidMerkleProof
    );

    if yes_or_no{

       item.votes_for() = iter
        .votes_for
        .checked_add(voting_power)
        .ok_or(ErrorCode::MathOverflow)?;

    } 

    else {
        iter.votes_against = iter
        .votes_against
        .checked_add(voting_power)
        .ok_or(ErrorCode::MathOverflow)?;
    }

   
    let receipt = &mut ctx.accounts.voter_receipt;
   
    receipt.proposal = proposal.key();
   
    receipt.voter = signer.key();

    receipt.voting_power = voting_power;

    receipt.bump = recepit_bump;

    if proposal.votes_for >= proposal.vote_required{

        proposal.proposal_status =  ProposalStatus::Passed ;       

        proposal.transfer_window = current_time;
    } 

    Ok(())

}