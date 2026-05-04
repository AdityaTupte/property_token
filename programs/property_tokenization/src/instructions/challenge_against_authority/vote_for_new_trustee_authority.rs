use anchor_lang::prelude::*;

use crate::{common::{AUTHORITY_CANDIDATE, AUTHORITYVOTERECEIPT, PROPERTY_SYSTEM_SEEDS, ProposalStatus,  REMOVETRUSTEEAUTHORITYPROPOSAL}, errors::ErrorCode, functions::voting_for_authority, state::{AuthorityCandidate, AuthorityVoteReceipt,  ElectAuthority, PropertySystemAccount}};


#[derive(Accounts)]
#[instruction(proposal_key:Pubkey,property_system_id:u64,candidate_key:Pubkey)]
pub struct VoteForNewTrusteeAuthority<'info>{


     #[account(
        mut,
    )]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds =[
            AUTHORITY_CANDIDATE,
            property_system.key().as_ref(), 
            removal_proposal.key().as_ref(),
            candidate_key.as_ref()  
        ],
        bump = authority_candidate.bump
    )]
    pub authority_candidate : Account<'info,AuthorityCandidate>,

    // #[account(
    //     seeds =[
    //         CHALLENGEAUTHORITY,
    //         &proposal.proposal_id.to_le_bytes(),
    //         property_system.key().as_ref(),
    //     ],
    //     bump = proposal.bump,
    //     constraint = proposal.status == ProposalStatus::Executed @ ErrorCode::ProposalNotExecuted
    // )]
    // pub proposal : Account<'info,ChallengeProposal>,

     #[account(
        mut,
        seeds=[
            REMOVETRUSTEEAUTHORITYPROPOSAL,
            property_system.key().as_ref(),
            proposal_key.as_ref(),
        ],
        bump=removal_proposal.bump,
        constraint = removal_proposal.snapshot_submitted @ ErrorCode::SnapshotNotSubmitted,
        constraint = removal_proposal.status == ProposalStatus::Active @ ErrorCode::ProposalNotActive

    )]
    pub removal_proposal: Account<'info,ElectAuthority>,


    #[account(
        seeds=[
            PROPERTY_SYSTEM_SEEDS,
            &property_system_id.to_le_bytes(),
        ],
        bump = property_system.bump
    )]
    pub property_system : Account<'info,PropertySystemAccount>,

    #[account(
        init,
        payer = signer,
        seeds=[
            AUTHORITYVOTERECEIPT,
            removal_proposal.key().as_ref(),
            signer.key().as_ref(),
            
        ],
        bump,
        space = 8 + AuthorityVoteReceipt::SIZE
    )]
    pub authority_vote_receipt: Account<'info, AuthorityVoteReceipt>,

    pub system_program :Program<'info,System>,
    
}

pub fn vote_for_new_trustee_authority(
    ctx:Context<VoteForNewTrusteeAuthority>,
    _proposal_key:Pubkey,_property_system_id:u64,_candidate_key:Pubkey,
    proof: Vec<[u8; 32]>,
    voting_power : u64,
)->Result<()>{

    voting_for_authority(
        ctx.accounts.removal_proposal.key(),
        &mut ctx.accounts.authority_candidate, 
        &mut ctx.accounts.authority_vote_receipt, 
        ctx.accounts.signer.key(), 
        &mut *ctx.accounts.removal_proposal,
        proof, 
        ctx.bumps.authority_vote_receipt, 
        voting_power,
        &ctx.accounts.property_system.governance_mint,
    )?;

    let remove_propsal =&mut ctx.accounts.removal_proposal;

    remove_propsal.rm_total_voting_power_gained = remove_propsal.rm_total_voting_power_gained
                                                                .checked_add(voting_power)
                                                                .ok_or(ErrorCode::MathOverflow)?;    

    Ok(())


}