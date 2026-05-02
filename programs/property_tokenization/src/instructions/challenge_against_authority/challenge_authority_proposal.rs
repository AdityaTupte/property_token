use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token_interface::{Mint, TokenAccount, TokenInterface}};

use crate::{common::{AuthorityType, CHALLENGEAUTHORITY, PROPERTY_SYSTEM_SEEDS, ProposalStatus, ProposalType}, errors::ErrorCode, state::{ChallengeProposal, PropertySystemAccount}};



#[derive(Accounts)]
#[instruction(proposal_id : u64,property_system_id : u64)]
pub struct ChallengeAuthorityProposal<'info>{

    #[account(mut)]
    pub signer : Signer<'info>,

    #[account(
        associated_token::mint = mint,
        associated_token::authority = signer,
        associated_token::token_program = token_program
    )]
    pub ata: InterfaceAccount<'info,TokenAccount>,

    #[account(
        init,
        payer = signer,
        seeds =[
            CHALLENGEAUTHORITY,
            property_system.key().as_ref(),
            &proposal_id.to_le_bytes(),
            
        ],
        bump,
        space = 8 + ChallengeProposal::SIZE ,
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

    pub system_program : Program<'info,System>,

    #[account(
        address = property_system.governance_mint @ ErrorCode::InvalidMint
    )]
    pub mint: InterfaceAccount<'info,Mint>,

    pub associated_token_program: Program<'info, AssociatedToken>,

    pub token_program :Interface<'info, TokenInterface>

}


pub fn challenge_authority(
    ctx:Context<ChallengeAuthorityProposal>,
    proposal_id : u64,
    _property_system_id : u64,
    charges_hash: [u8; 32],
    evidence_hash : [u8;32],
    // authority_type : AuthorityType,
) -> Result<()>{

    require!(ctx.accounts.ata.amount > 0 , ErrorCode::InsufficentBalance);

    let proposal = &mut ctx.accounts.proposal;

    let current_time =  Clock::get()?.unix_timestamp;

    proposal.creator = ctx.accounts.signer.key();

    proposal.proposal_id = proposal_id;   
  
    proposal.required_vote_to_active = ((ctx.accounts.mint.supply as u128)
                                .checked_mul(10)
                                .and_then(|v| v.checked_add(99)) 
                                .and_then(|v| v.checked_div(100))
                                .ok_or(ErrorCode::MathOverflow)?) as u64;

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