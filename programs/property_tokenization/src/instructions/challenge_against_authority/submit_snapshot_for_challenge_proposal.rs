use anchor_lang::prelude::*;

use crate::{common::{CHALLENGEAUTHORITY, PROPERTY_SYSTEM_SEEDS, ProposalStatus}, errors::ErrorCode, state::{ChallengeProposal, PropertySystemAccount}};

#[derive(Accounts)]
#[instruction(proposal_id : u64,property_system_id : u64)]
pub struct SubmitSnaphotForChallengeProposal<'info>{

    #[account(
        constraint = proposal.creator == signer.key() @ ErrorCode::UnAuthorized
    )]
    pub signer : Signer<'info>,

    #[account(
        mut,
        seeds =[
            CHALLENGEAUTHORITY,
            property_system.key().as_ref(),
            &proposal_id.to_le_bytes(),
        ],
        bump = proposal.bump,
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

}


pub fn submit_snapshot_for_challenge_proposal(
    ctx: Context<SubmitSnaphotForChallengeProposal>,
    _proposal_id : u64,_property_system_id : u64,
    merkle_root : [u8;32],
    
)->Result<()>{

    let proposal = &mut ctx.accounts.proposal;
    
    let current_time =  Clock::get()?.unix_timestamp;

    require!(proposal.status == ProposalStatus::Draft , ErrorCode::NotInDraft);
    

    proposal.voting_deadline = current_time
                                    .checked_add(24*60*60*3 as i64 )
                                    .ok_or(ErrorCode::MathOverflow)?;

    proposal.merkle_root =  merkle_root;

    proposal.status = ProposalStatus::Active;

   
    Ok(())

}