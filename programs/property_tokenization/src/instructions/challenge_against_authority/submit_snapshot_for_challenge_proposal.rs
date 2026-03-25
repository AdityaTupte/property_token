use anchor_lang::prelude::*;

use crate::{common::{CHALLENGEAUTHORITY, PROPERTY_SYSTEM_SEEDS, ProposalStatus}, errors::ErrorCode, state::{ChallengeProposal, PropertySystemAccount}};

#[derive(Accounts)]


pub struct SubmitSnaphotForChallengeProposal<'info>{

    #[account(
        constraint = proposal.creator == signer.key() @ ErrorCode::NotAuthorized
    )]
    pub signer : Signer<'info>,

    #[account(
        seeds =[
            CHALLENGEAUTHORITY,
            &proposal.proposal_id.to_le_bytes(),
            property_system.key().as_ref(),
        ],
        bump,
        constraint = proposal.status == ProposalStatus::Draft @ ErrorCode::NotInDraft
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

}


pub fn submit_snapshot_for_challenge_proposal(
    ctx: Context<SubmitSnaphotForChallengeProposal>,
    merkle_root : [u8;32]
)->Result<()>{

    let current_time =  Clock::get()?.unix_timestamp;

    let proposal = &mut ctx.accounts.proposal;

    proposal.voting_deadline = current_time
                                    .checked_add(24*60*60*3 as i64 )
                                    .ok_or(ErrorCode::MathOverflow)?;

    proposal.merkle_root =  merkle_root;

    proposal.status = ProposalStatus::Active;

    Ok(())

}