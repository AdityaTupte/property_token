use anchor_lang::prelude::*;

use crate::{common::{AUTHORITY_CANDIDATE, AuthorityType, PROPERTY_SYSTEM_SEEDS, ProposalStatus, RANKCOUNT, RANKINGACCOUNT, REMOVEARBITRARAUTHORITYPROPOSAL, }, errors::ErrorCode, functions::add_new_authority, state::{AuthorityCandidate, ElectAuthority, PropertySystemAccount, RankCounter, RankingAccount,}};


#[derive(Accounts)]

#[instruction(proposal_key:Pubkey, candidate_key:Pubkey,property_system_id:u64,proposal_id:u64,ranking:u8)]
pub struct AddNewArbitrarAuthority<'info>{

    #[account(mut)]
    pub signer: Signer<'info>,

      #[account(
        mut,
        seeds=[
            REMOVEARBITRARAUTHORITYPROPOSAL,
            property_system.key().as_ref(),
            proposal_key.as_ref(),
        ],
        bump = removal_proposal.bump,
        constraint = removal_proposal.snapshot_submitted @ ErrorCode::SnapshotNotSubmitted,
        constraint = removal_proposal.status == ProposalStatus::Approved @ ErrorCode::ProposalNotApproved,
        constraint = removal_proposal.authority_type == AuthorityType::ARBITRATOR @ ErrorCode::AuthorityNotMatchWithProposal
    )]
    pub removal_proposal : Account<'info,ElectAuthority>,

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
        seeds=[
            PROPERTY_SYSTEM_SEEDS,
            &property_system_id.to_le_bytes(),
        ],
        bump = property_system.bump
    )]
    pub property_system:Account<'info,PropertySystemAccount>,

    //     #[account(
    //     seeds=[
    //         TRUSTEEREGISTRYSEEDS,
    //         property_system.key().as_ref()
    //     ],
    //     bump = trustee_registry.bump
    // )]
    // pub trustee_registry : Account<'info,TrusteeRegistry>,

        #[account(
            mut,
        seeds=[
            AUTHORITY_CANDIDATE,
            property_system.key().as_ref(), 
            removal_proposal.key().as_ref(),
            candidate_key.as_ref(),
        ],
        bump = authority_candidate.bump
    )]
    pub authority_candidate: Account<'info,AuthorityCandidate>,

      #[account(
        init_if_needed,
        payer = signer,
        seeds =[
            RANKCOUNT,
            removal_proposal.key().as_ref(),
        ],
        bump,
        space = 8 + RankCounter::SIZE
    )]
    pub counter : Account<'info,RankCounter>,


    #[account(
        init,
        payer=signer,
        seeds=[
            RANKINGACCOUNT,
            &ranking.to_le_bytes(),
            removal_proposal.key().as_ref(),
            property_system.key().as_ref()
        ],
        bump,
        space=8 + RankingAccount::SIZE
    )]
    pub ranking_acc : Account<'info,RankingAccount>,

    pub system_program:Program<'info,System>,


}


pub fn add_new_arbitrar_authority_for_remove_proposal(
    ctx:Context<AddNewArbitrarAuthority>,
    _proposal_key:Pubkey,
    _candidate_key:Pubkey,
    _property_system_id:u64,
    _proposal_id:u64,
    ranking:u8,
)->Result<()>{

    let proposal_key = ctx.accounts.removal_proposal.key();
    

     add_new_authority(
        &mut *ctx.accounts.removal_proposal,
        &mut ctx.accounts.ranking_acc,
        &mut ctx.accounts.authority_candidate,
        ranking,
        proposal_key,
        ctx.bumps.ranking_acc,
        &mut ctx.accounts.counter,
    )?;





    

    Ok(())



}
