use anchor_lang::prelude::*;

use crate::{common::{AUTHORITY_CANDIDATE, ELECT_ARBITRAR, PROPERTY_SYSTEM_SEEDS, ProposalStatus, RANKCOUNT, RANKINGACCOUNT, TRUSTEE_RECEIPT_SEEDS}, errors::ErrorCode, functions::add_new_authority, state::{AuthorityCandidate, ElectAuthority, PropertySystemAccount, RankCounter, RankingAccount, TrusteeRecepit}};


#[derive(Accounts)]
#[instruction(candidate_key:Pubkey,property_system_id:u64,proposal_id:u64,ranking:u8)]
pub struct AddNewArbitrar<'info>{

    #[account(
        mut,
        // constraint = trustee_registry.trustees.contains(&trustee.key()) @ ErrorCode::NotAuthorized
    )]
    pub signer: Signer<'info>,


    #[account(
        seeds = [
            TRUSTEE_RECEIPT_SEEDS,
            property_system.key().as_ref(),
            signer.key().as_ref()
        ],
        bump = trustee_receipt.bump,
    )]
    pub trustee_receipt: Account<'info,TrusteeRecepit>,

    #[account(
        mut,
        seeds=[
            ELECT_ARBITRAR,
            property_system.key().as_ref(),
            &proposal_id.to_le_bytes()
        ],
        bump = proposal.bump,
        constraint = proposal.snapshot_submitted @ ErrorCode::SnapshotNotSubmitted,
        constraint = proposal.status == ProposalStatus::Active @ ErrorCode::ProposalNotActive
    )]
    pub proposal : Account<'info,ElectAuthority>,

    #[account(
        seeds=[
            PROPERTY_SYSTEM_SEEDS,
            &property_system_id.to_le_bytes(),
        ],
        bump = property_system.bump
    )]
    pub property_system:Account<'info,PropertySystemAccount>,

    // #[account(
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
            proposal.key().as_ref(),
            candidate_key.as_ref(), 
        ],
        bump = authority_candidate.bump,
        constraint  = !authority_candidate.selected @ ErrorCode::AuthoritySelected ,
    )]
    pub authority_candidate: Account<'info,AuthorityCandidate>,


    #[account(
        init_if_needed,
        payer = signer,
        seeds =[
            RANKCOUNT,
            proposal.key().as_ref(),
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
            proposal.key().as_ref(),
            property_system.key().as_ref()
        ],
        bump,
        space=8 + RankingAccount::SIZE
    )]
    pub ranking_acc : Account<'info,RankingAccount>,

    // #[account(
    //     init,
    //     payer = signer,
    //     seeds=[
    //         candidate_key.as_ref(),
    //         proposal.key().as_ref(),
    //     ],
    //     bump,
    //     space = 8 + AuthorityCandidateSelectionRecipt::SIZE
    // )]
    // pub authority_candiate_selection_receipt : Account<'info,AuthorityCandidateSelectionRecipt>,

    pub system_program : Program<'info,System>


} 


pub fn add_new_arbitrar(
    ctx:Context<AddNewArbitrar>,
    _candidate_key:Pubkey,
    _property_system_id:u64,
    _proposal_id:u64,
    ranking:u8,
)->Result<()>{

    let proposal_key = ctx.accounts.proposal.key();

    add_new_authority(
        &mut *ctx.accounts.proposal,
        &mut ctx.accounts.ranking_acc,
        &mut ctx.accounts.authority_candidate,
        ranking,
        proposal_key,
        ctx.bumps.ranking_acc,
        &mut ctx.accounts.counter,
        // &mut ctx.accounts.authority_candiate_selection_receipt,
        // ctx.bumps.authority_candiate_selection_receipt
    )?;



    

    Ok(())
}