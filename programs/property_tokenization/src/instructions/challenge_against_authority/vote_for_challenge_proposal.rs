use anchor_lang::prelude::*;
use anchor_spl::{associated_token::spl_associated_token_account::solana_program::keccak,};
use crate::{common::{CHALLENGEAUTHORITY, PROPERTY_SYSTEM_SEEDS, ProposalStatus}, errors::ErrorCode, functions::verify_proof, state::{ChallengeProposal, PropertySystemAccount}};


#[derive(Accounts)]

pub struct VoteForChallengeProposal<'info>{

    pub signer : Signer<'info>,

     #[account(
        seeds =[
            CHALLENGEAUTHORITY,
            &proposal.proposal_id.to_le_bytes(),
            property_system.key().as_ref(),
        ],
        bump,
        constraint = proposal.status == ProposalStatus::Draft @ ErrorCode::NotInDraft
    )]
    pub proposal : Account<'info,ChallengeProposal>,


    #[account(
        seeds=[
            PROPERTY_SYSTEM_SEEDS,
            &property_system.property_system_id.to_le_bytes()
        ],
        bump = property_system.bump,
    )]
    pub property_system : Account<'info,PropertySystemAccount>,

}


pub fn vote_for_challenge_proposal(
    ctx:Context<VoteForChallengeProposal>,
    proof: Vec<[u8;32]>,
    voting_power : u64,
)->Result<()>{

    

    let proposal = &mut ctx.accounts.proposal;

    require!(Clock::get()?.unix_timestamp < proposal.voting_deadline, ErrorCode::VotingPeriodExpired );

    let property_system = & ctx.accounts.property_system;

    let leaf = keccak::hashv(&[
            &[proposal.proposal_type as u8],
            ctx.accounts.signer.key().as_ref(),
            proposal.key().as_ref(),
            property_system.key().as_ref(),
            property_system.governance_mint.as_ref(),
            &voting_power.to_le_bytes(),               
        ]).0 ;

    require!(verify_proof(leaf, &proof, proposal.merkle_root) , ErrorCode::InvalidMerkleProof);
    
    proposal.vote_gained = proposal.vote_gained
                                    .checked_add(voting_power)
                                    .ok_or(ErrorCode::MathOverflow)?;

    if  proposal.vote_gained > proposal.required_vote_to_active {

        proposal.status = ProposalStatus::Passed;

    }

    Ok(())


}