use anchor_lang::prelude::*;

use crate::{common::{AUTHORITY_CANDIDATE, ProposalStatus, RANKINGACCOUNT, }, errors::ErrorCode, state::{AuthorityCandidate, ElectAuthority, RankingAccount}};


#[derive(Accounts)]
#[instruction(proposal_key:Pubkey,property_system:Pubkey,candidate_key1:Pubkey,candidate_key2:Pubkey,ranking1:u8,ranking2:u8)]
pub struct AdjustAuthorityRanking<'info>{

    #[account()]
    pub signer: Signer<'info>,

    #[account(
        mut,
        // seeds=[
        //     REMOVEAUTHORITY,
        //     property_system.key().as_ref(),
        //     proposal_key.as_ref(),
        // ],
        // bump = removal_proposal.bump,
        constraint = removal_proposal.snapshot_submitted @ ErrorCode::SnapshotNotSubmitted,
        constraint = removal_proposal.status == ProposalStatus::Approved @ ErrorCode::ProposalNotActive
    )]
    pub removal_proposal : Account<'info,ElectAuthority>,

    #[account(
        seeds=[
            AUTHORITY_CANDIDATE,
            property_system.key().as_ref(), 
            removal_proposal.key().as_ref(),
            candidate_key1.as_ref(), 
        ],
        bump = authority_candidate1.bump,
    )]
    pub authority_candidate1: Account<'info,AuthorityCandidate>,


    // #[account(
    //     seeds=[
    //         candidate_key1.as_ref(),
    //         proposal.key().as_ref(),
    //     ],
    //     bump = authority_candiate_selection_receipt1.bump,
    // )]
    // pub authority_candiate_selection_receipt1 : Account<'info,AuthorityCandidateSelectionRecipt>,


    #[account(
        mut,
        seeds=[
            RANKINGACCOUNT,
            &ranking1.to_le_bytes(),
            removal_proposal.key().as_ref(),
            property_system.key().as_ref()
        ],
        bump = ranking_acc1.bump,
        // constraint = ranking_acc1.candidate_key == authority_candiate_selection_receipt1.candidate_key @ ErrorCode::RankAccountInvalid
    )]
    pub ranking_acc1 : Account<'info,RankingAccount>,



    #[account(
        seeds=[
            AUTHORITY_CANDIDATE,
            property_system.key().as_ref(), 
            removal_proposal.key().as_ref(),
            candidate_key2.as_ref(), 
        ],
        bump = authority_candidate2.bump
    )]
    pub authority_candidate2: Account<'info,AuthorityCandidate>,


    // #[account(
    //     seeds=[
    //         candidate_key2.as_ref(),
    //         proposal.key().as_ref(),
    //     ],
    //     bump = authority_candiate_selection_receipt2.bump,
    // )]
    // pub authority_candiate_selection_receipt2 : Account<'info,AuthorityCandidateSelectionRecipt>,

    #[account(
        mut,
        seeds=[
            RANKINGACCOUNT,
            &ranking2.to_le_bytes(),
            removal_proposal.key().as_ref(),
            property_system.key().as_ref()
        ],
        bump = ranking_acc2.bump,
        // constraint = ranking_acc2.candidate_key == authority_candiate_selection_receipt2.candidate_key @ ErrorCode::RankAccountInvalid
    )]
    pub ranking_acc2 : Account<'info,RankingAccount>,

}


pub fn adjust_arbitrar_ranking(
    ctx:Context<AdjustAuthorityRanking>,
    _proposal_key:Pubkey,
    _property_system:Pubkey,
    _candidate_key1:Pubkey,
    _candidate_key2:Pubkey,
    _ranking1:u8,
    _ranking2:u8
)->Result<()>{

    // let auth_candidate1 = &mut ctx.accounts.authority_candiate_selection_receipt1;

    // let auth_candidate2 = &mut ctx.accounts.authority_candiate_selection_receipt2;

    let rank1 = &mut ctx.accounts.ranking_acc1;

    let rank2 = &mut ctx.accounts.ranking_acc2;

    require!(rank1.key() != rank2.key() ,ErrorCode::SameRankAccount );

    // require!(auth_candidate1.proposal == auth_candidate2.proposal,ErrorCode::InvalidProposal);

    let temp_rank_2 = rank2.rank.checked_add(1).ok_or(ErrorCode::MathOverflow)?;

    require!(rank1.rank == temp_rank_2 ,ErrorCode::RankAccountNotAdjacent);

    let authority_candidate_acc1 = &mut ctx.accounts.authority_candidate1;

    let authority_candidate_acc2 = &mut ctx.accounts.authority_candidate2;

    // require!(authority_candidate_acc1.key() != authority_candidate_acc2.key(), ErrorCode::SameAuthority)

    require!(authority_candidate_acc1.vote_gained > authority_candidate_acc2.vote_gained, ErrorCode::NotRankChangeRequired );

    let temp = rank1.candidate_key;

    rank1.candidate_key = rank2.candidate_key;

    rank2.candidate_key = temp;

    Ok(())

}