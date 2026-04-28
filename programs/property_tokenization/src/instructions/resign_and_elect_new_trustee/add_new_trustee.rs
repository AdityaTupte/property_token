use anchor_lang::prelude::*;

use crate::{common::{AUTHORITY_CANDIDATE, ELECT_TRUSTEE, PROPERTY_SYSTEM_SEEDS, ProposalStatus, RANKINGACCOUNT, TRUSTEE_RECEIPT_SEEDS}, errors::ErrorCode, functions::add_new_authority, state::{AuthorityCandidate, ElectAuthority, PropertySystemAccount, RankingAccount, TrusteeRecepit}};


#[derive(Accounts)]
#[instruction(candidate_key:Pubkey,property_system_id:u64,proposal_id:u64,ranking:u8)]
pub struct AddNewTrustee<'info>{

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
            ELECT_TRUSTEE,
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
        seeds=[
            AUTHORITY_CANDIDATE,
            property_system.key().as_ref(), 
            proposal.key().as_ref(),
            candidate_key.as_ref(), 
        ],
        bump = authority_candidate.bump
    )]
    pub authority_candidate: Account<'info,AuthorityCandidate>,


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

    pub system_program : Program<'info,System>


} 


pub fn add_new_trustee(
    ctx:Context<AddNewTrustee>,
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
    )?;



    

    Ok(())
}