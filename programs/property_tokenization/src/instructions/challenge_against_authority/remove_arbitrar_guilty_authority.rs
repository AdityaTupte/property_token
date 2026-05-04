use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::{
    common::{
        AuthorityType, ProposalStatus,  CHALLENGEAUTHORITY, PROPERTY_SYSTEM_SEEDS,
         REMOVEARBITRARAUTHORITYPROPOSAL,
    },
    errors::ErrorCode,
    state::{ChallengeProposal, ElectAuthority, PropertySystemAccount, 
}};

#[derive(Accounts)]
#[instruction(proposal_id:u64,property_system_id:u64)]
pub struct RemoveGuiltyArbitrarAuthority<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        associated_token::mint = mint,
        associated_token::authority = signer,
        associated_token::token_program = token_program,
    )]
    pub ata: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        seeds = [
            CHALLENGEAUTHORITY,
            property_system.key().as_ref(),
            &proposal_id.to_le_bytes(),
        ],
        bump = proposal.bump,
        constraint = proposal.status == ProposalStatus::Executed @ ErrorCode::ProposalNotExecuted
    )]
    pub proposal: Box<Account<'info, ChallengeProposal>>,

    #[account(
        seeds=[
            PROPERTY_SYSTEM_SEEDS,
            &property_system_id.to_le_bytes()
        ],
        bump = property_system.bump,
    )]
    pub property_system: Box<Account<'info, PropertySystemAccount>>,

    // #[account(
    //     init,
    //     payer = signer,
    //     seeds=[
    //         PROPOSEREMOVEARBITRARPROPOSAL,
    //         property_system.key().as_ref(),
    //         proposal.key().as_ref(),
    //     ],
    //     bump,
    //     space = 8 + ProposeRemoveProposal::SIZE
    // )]
    // pub propose_remove_proposal: Box<Account<'info, ProposeRemoveProposal>>,

    #[account(
        init,
        payer = signer,
        seeds=[
            REMOVEARBITRARAUTHORITYPROPOSAL,
            property_system.key().as_ref(),
            proposal.key().as_ref(),
        ],
        bump,
        space = 8 + ElectAuthority::SIZE,
    )]
    pub removal_proposal: Box<Account<'info, ElectAuthority>>,

    pub system_program: Program<'info, System>,

    #[account(
        address = property_system.governance_mint @ ErrorCode::InvalidMint
    )]
    pub mint: Box<InterfaceAccount<'info, Mint>>,

    pub associated_token_program: Program<'info, AssociatedToken>,

    pub token_program: Interface<'info, TokenInterface>,
}

pub fn removal_of_arbitrar_proposal(
    ctx: Context<RemoveGuiltyArbitrarAuthority>,
    _proposal_id: u64,
    _property_system_id: u64,
) -> Result<()> {
   let proposal = &ctx.accounts.proposal;

    let remove_proposal = &mut ctx.accounts.removal_proposal;

    let removal_proposal_deadline = proposal.result_time
                                            .checked_add(24*60*60*3)
                                            .ok_or(ErrorCode::MathOverflow)?;

    require!(proposal.result_time < removal_proposal_deadline , ErrorCode::RemovalProposalDeadline );

    require!(ctx.accounts.ata.amount > 0 , ErrorCode::InsufficentBalance);

    let supply = ctx.accounts.mint.supply;

    

    let numerator = supply
    .checked_mul(30)
    .ok_or(ErrorCode::MathOverflow)?
    .checked_add(99) 
    .ok_or(ErrorCode::MathOverflow)?;

    let threshold = numerator
        .checked_div(100)
        .ok_or(ErrorCode::MathOverflow)?;

    remove_proposal.rm_voting_threshold = threshold   ;
    

    remove_proposal.property_system = ctx.accounts.property_system.key();

    // remove_proposal.total_authority_to_resign += 1;

    remove_proposal.authority_type = AuthorityType::ARBITRATOR;

    remove_proposal.status = ProposalStatus::Draft;

    remove_proposal.proposal_id = proposal.proposal_id;

    remove_proposal.bump = ctx.bumps.removal_proposal;


    Ok(())
}
