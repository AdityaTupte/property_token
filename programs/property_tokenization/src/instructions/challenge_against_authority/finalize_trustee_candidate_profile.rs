use anchor_lang::prelude::{ *};

use crate::{common::{ARBITRAR_RECEIPT_SEEDS, CANDIDATE_PROFILE, CHALLENGEAUTHORITY, OFFENDERRECEIPT, PROPERTY_SYSTEM_SEEDS, ProposalStatus, TRUSTEE_RECEIPT_SEEDS}, errors::ErrorCode, state::{CandidateProfile, ChallengeProposal, OffenderReceipt, PropertySystemAccount, trustee_recepit}};

#[derive(Accounts)]
#[instruction(proposal_id:u64,property_system_id:u64,candidate_key:Pubkey)]
pub struct FinalizeTrusteeCandidateProfile<'info>{

    
    pub signer : Signer<'info>,

     #[account(
        seeds =[
            CHALLENGEAUTHORITY,
            property_system.key().as_ref(),
            &proposal_id.to_le_bytes(),
        ],
        bump = proposal.bump,
        constraint = proposal.status == ProposalStatus::Executed @ ErrorCode::ProposalNotExecuted
    )]
    pub proposal : Account<'info,ChallengeProposal>,

    #[account(
        seeds=[
            PROPERTY_SYSTEM_SEEDS,
            &property_system_id.to_le_bytes()
        ],
        bump = property_system.bump,
    )]
    pub property_system : Account<'info,PropertySystemAccount>,

   #[account(
    mut,
        seeds=[
                CANDIDATE_PROFILE,
                candidate_key.as_ref()
        ],
        bump = candidate_profile.bump,
        
    )]
    pub candidate_profile : Account<'info,CandidateProfile>,

    #[account(
        mut,
        seeds=[
            OFFENDERRECEIPT,
            TRUSTEE_RECEIPT_SEEDS,
            property_system.key().as_ref(),
            proposal.key().as_ref(),
            candidate_profile.candidate.as_ref(),
        ],
        bump = trustee_offender_receipt.bump,
        constraint = !trustee_offender_receipt.is_finalized @ ErrorCode::AlreadyFinalized 
    )]
    pub trustee_offender_receipt :Account<'info,OffenderReceipt>,

   


}

pub fn finalize_trustee_candidate_profile(
    ctx:Context<FinalizeTrusteeCandidateProfile>,
    _proposal_id:u64,_property_system_id:u64,_candidate_key:Pubkey
)->Result<()>{

    let candidate = &mut ctx.accounts.candidate_profile;

    let proposal = &mut ctx.accounts.proposal;

    let receipt = &mut ctx.accounts.trustee_offender_receipt;

    // let index  = proposal.index;

    // require!(proposal.against.len() > index as usize ,ErrorCode::AlreadyFinalized);

    // require!(proposal.against.contains(&candidate.key()),ErrorCode::AuthorityNotFound);

//     require!(
//     proposal.against.get(index as usize) == Some(&candidate.key()),
//     ErrorCode::ChangeCandidateFinalization
// );

    
    if candidate.actions_history < proposal.guilty {
        candidate.actions_history = proposal.guilty;
    }

    receipt.is_finalized = true;

    


    ////emit

    Ok(())
}