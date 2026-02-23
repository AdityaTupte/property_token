use anchor_lang::{ prelude::*};
use anchor_spl::associated_token::spl_associated_token_account::solana_program::keccak;

use crate::{constant::{ProposalStatus, TRANSFERPROPOSAL, VOTERRECIEPT}, errors::ErrorCode, functions::verify_proof, state::{PropertySystemAccount, TransferLandDetail, VoterReciept}};

#[derive(Accounts)]
pub struct Voting<'info>{

    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds=[
            TRANSFERPROPOSAL,
            &proposal.proposal_id.to_le_bytes(),
            property_system.key().as_ref(),
        ],
        bump = proposal.bump,
         constraint = proposal.snapshot_submitted @ ErrorCode::SnapshotNotSubmitted,
         constraint = proposal.proposal_status !=  ProposalStatus::Passed as u8 @ ErrorCode::ProposalAlreadyPassed,
         constraint = proposal.proposal_status == ProposalStatus::Active as u8 @ ErrorCode::ProposalNotActive
    )]

    pub proposal : Account<'info,TransferLandDetail>,


    #[account(
        constraint = proposal.source_property_system == property_system.key() @ ErrorCode::InvalidProposal
    )]
    pub property_system: Account<'info,PropertySystemAccount>,

    #[account(
        init,
        payer = signer,
        seeds=[
            VOTERRECIEPT,
            proposal.key().as_ref(),
            signer.key().as_ref(),
        ],
        bump,
        space =  8 + VoterReciept::SIZE
    )]

    pub voter_receipt : Account<'info,VoterReciept>,

    pub system_program : Program<'info,System>,

}

    pub fn voteToTransfer(
        ctx:Context<Voting>,
        proof: Vec<[u8; 32]>,
        voting_power : u64,
        yes_or_no : bool,
    )->Result<()>{

        


        Ok(())





    }