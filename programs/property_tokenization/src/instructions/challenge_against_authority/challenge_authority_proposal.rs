use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token_interface::{Mint, TokenAccount}};

use crate::{common::{CHALLENGEAUTHORITY, PROPERTY_SYSTEM_SEEDS, ProposalStatus, ProposalType}, errors::ErrorCode, state::{ChallengeProposal, PropertySystemAccount}};





#[derive(Accounts)]
#[instruction(proposal_id : u64)]
pub struct ChallengeAuthorityProposal<'info>{

    #[account(mut)]
    pub signer : Signer<'info>,

    #[account(
        associated_token::mint = mint,
        associated_token::authority = signer,
    )]
    pub ata: InterfaceAccount<'info,TokenAccount>,

    #[account(
        init,
        payer = signer,
        seeds =[
            CHALLENGEAUTHORITY,
            &proposal_id.to_le_bytes(),
            property_system.key().as_ref(),
        ],
        bump,
        space = 8 + ChallengeProposal::SIZE ,
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


    pub system_program : Program<'info,System>,

    #[account(
        address = property_system.governance_mint @ ErrorCode::InvalidMint
    )]
    pub mint: InterfaceAccount<'info,Mint>,

    pub associated_token_program: Program<'info, AssociatedToken>

}


pub fn challenge_authority(
    ctx:Context<ChallengeAuthorityProposal>,
    charges_hash: [u8; 32],
    evidence_hash : [u8;32],
    authorities : Vec<Pubkey>,
    proposal_id : u64,
    
) -> Result<()>{

    require!(ctx.accounts.ata.amount > 0 , ErrorCode::InsufficentBalance);

    let proposal = &mut ctx.accounts.proposal;

    let current_time =  Clock::get()?.unix_timestamp;

    proposal.creator = ctx.accounts.signer.key();

    proposal.proposal_id = proposal_id;

    proposal.against = authorities;

    proposal.required_vote_to_active =  ((ctx.accounts.mint.supply as u128) * 10 / 100) as u64;
  
    proposal.charges_hash = charges_hash;

    proposal.evidence_hash = evidence_hash;


    proposal.created_at = current_time;
    
    proposal.status = ProposalStatus::Draft;

    proposal.property_system = ctx.accounts.property_system.key();

    proposal.bump = ctx.bumps.proposal;

    proposal.proposal_type = ProposalType::CHALLLENGEAUTHORITY;

 

    
    ///// emit


Ok(())

}