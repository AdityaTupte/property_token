use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token_interface::{Mint, TokenAccount}};

use crate::{common::{CHALLENGEAUTHORITY, PROPERTY_SYSTEM_SEEDS, ProposalStatus, REMOVEAUTHORITY}, errors::ErrorCode, state::{ChallengeProposal, ElectAuthority, PropertySystemAccount}};

#[derive(Accounts)]

pub struct RemoveGuiltyAuthority<'info>{

    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        associated_token::mint = mint,
        associated_token::authority = signer,
    )]
    pub ata : InterfaceAccount<'info,TokenAccount>,

    #[account(
        seeds =[
            CHALLENGEAUTHORITY,
            &proposal.proposal_id.to_le_bytes(),
            property_system.key().as_ref(),
        ],
        bump = proposal.bump,
        constraint = proposal.status == ProposalStatus::Executed @ ErrorCode::ProposalNotExecuted
    )]
    pub proposal : Account<'info,ChallengeProposal>,


    #[account(
            seeds=[
            PROPERTY_SYSTEM_SEEDS,
            &property_system.property_system_id.to_le_bytes()
        ],
        bump = property_system.bump,
        
    )]
    pub property_system: Account<'info,PropertySystemAccount>, 
    
    #[account(
        init,
        payer = signer,
        seeds=[
            REMOVEAUTHORITY,
            proposal.key().as_ref(),
            property_system.key().as_ref(),
        ],
        bump,
        space = 8 + ElectAuthority::SIZE, 
    )]
    pub removal_proposal : Account<'info,ElectAuthority>,

    pub system_program : Program<'info,System>,

     #[account(
        address = property_system.governance_mint @ ErrorCode::InvalidMint
    )]
    pub mint: InterfaceAccount<'info,Mint>,

    pub associated_token_program: Program<'info, AssociatedToken>
    

}

pub fn removal_proposal(
    ctx:Context<RemoveGuiltyAuthority>
)->Result<()>{

    let proposal = &ctx.accounts.proposal;

    let remove_proposal = &mut ctx.accounts.removal_proposal;

    let removal_proposal_deadline = proposal.result_time
                                            .checked_add(24*60*60*3)
                                            .ok_or(ErrorCode::MathOverflow)?;

    require!(proposal.result_time < removal_proposal_deadline , ErrorCode::RemovalProposalDeadline );

    require!(ctx.accounts.ata.amount > 0 , ErrorCode::InsufficentBalance);

    remove_proposal.property_system = ctx.accounts.property_system.key();

    remove_proposal.authority_to_resign = proposal.against.clone();

    remove_proposal.authority_type = proposal.authority_type;

    remove_proposal.status = ProposalStatus::Draft;

    remove_proposal.proposal_id = proposal.proposal_id;

    remove_proposal.bump = ctx.bumps.removal_proposal;


    Ok(())
}