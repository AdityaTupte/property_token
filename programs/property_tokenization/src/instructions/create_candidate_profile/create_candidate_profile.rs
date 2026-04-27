use anchor_lang::prelude::*;

use crate::{common::{CANDIDATE_PROFILE, HARDCODED_PUBKEY, ReasonType}, state::CandidateProfile};



#[derive(Accounts)]


pub struct CreateCandidateProfile<'info>{

    #[account(
        mut,
     address = HARDCODED_PUBKEY, 
    )]
    pub signer : Signer<'info>,


    pub candidate:SystemAccount<'info>,

    #[account(
        init,
        payer = signer,
        seeds=[
            CANDIDATE_PROFILE,
            candidate.key().as_ref()
        ],
        bump,
        space = 8 + CandidateProfile::SIZE
    )]
    pub candidate_profile : Account<'info,CandidateProfile>,

    pub system_program : Program<'info,System>,

}


pub fn create_candidate_profile(
    ctx:Context<CreateCandidateProfile>,
    metadata_hash : [u8;32]
)->Result<()>{

    let candidate_profile = &mut ctx.accounts.candidate_profile;

    candidate_profile.candidate = ctx.accounts.candidate.key();

    // candidate_profile.is_verified = true;

    candidate_profile.total_applied = 0;

    candidate_profile.total_selected_as_trustee = 0;

    candidate_profile.total_selected_as_arbitrar = 0;

    candidate_profile.is_blacklisted = false;

    candidate_profile.removal_count = 0;

    candidate_profile.actions_history = ReasonType::None;

    candidate_profile.metadata_hash = metadata_hash;

    candidate_profile.bump = ctx.bumps.candidate_profile;

    Ok(())



}