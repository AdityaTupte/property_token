use anchor_lang::prelude::*;

use crate::{common::{CHALLENGEAUTHORITY, PROPERTY_SYSTEM_SEEDS, ProposalStatus, REMOVEAUTHORITY}, errors::ErrorCode, functions::submit_authority, state::{ChallengeProposal, ElectAuthority, PropertySystemAccount}};



#[derive(Accounts)]
pub struct SubmitSnapshotForGuiltyAuthority<'info>{

    
    pub signer: Signer<'info>,

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
            REMOVEAUTHORITY,
            proposal.key().as_ref(),
            property_system.key().as_ref(),
        ],
        bump=removal_proposal.bump,
        constraint = removal_proposal.status == ProposalStatus::Draft @ ErrorCode::NotInDraft
    )]
    pub removal_proposal: Account<'info,ElectAuthority>,

      #[account(
            seeds=[
            PROPERTY_SYSTEM_SEEDS,
            &property_system.property_system_id.to_le_bytes()
        ],
        bump = property_system.bump,
        
    )]
    pub property_system: Account<'info,PropertySystemAccount>, 


}


pub fn submit_snapshot_for_guilty_authority(
    ctx:Context<SubmitSnapshotForGuiltyAuthority>,
    merkle_root : [u8;32],
)->Result<()>{


    let proposal = &mut *ctx.accounts.removal_proposal;


                                            
submit_authority(
    proposal,
    merkle_root,
    3,3,3,3)?;
    Ok(())

}