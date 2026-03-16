use anchor_lang::prelude::*;

use crate::{common::RT_CHG_PROPOSAL_SEEDS, functions::finalize, state::RTChgProposal};



#[derive(Accounts)]
pub struct Finalize<'info>{


    #[account()]
    pub signer:Signer<'info>,


    #[account(
        seeds=[
            RT_CHG_PROPOSAL_SEEDS,
            &proposal.proposal_id.to_le_bytes(),
            proposal.property_system.key().as_ref()
        ],
        bump = proposal.bump
    )]
    pub proposal : Account<'info,RTChgProposal>
}


pub fn finalize_rtc_proposal(ctx:Context<Finalize>)->Result<()>{

    let proposal = &mut *ctx.accounts.proposal; 


    let voting_end_time = proposal.threshold_submission_deadline + ( 24 * 60 *60  * 3 );

    let new_threshold_submission_deadline = proposal.threshold_submission_deadline + ( 24 * 60 *60  * 4 );

    let challenge_end_time = proposal.threshold_submission_deadline + ( 24 * 60 *60  * 6 );

    finalize(proposal)?;

    proposal.voting_for_threshold_deadline = voting_end_time;

    proposal.add_new_threshold_deadline = new_threshold_submission_deadline;

    proposal.challenge_new_threshold_deadline = challenge_end_time; 

    Ok(())
}