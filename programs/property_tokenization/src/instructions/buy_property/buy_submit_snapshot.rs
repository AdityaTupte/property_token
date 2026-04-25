use anchor_lang::prelude::*;
use crate::common::{BUYPROPERTY, HARDCODED_PUBKEY, ProposalStatus};
use crate::errors::ErrorCode;
use crate::functions::submit;
use crate::state::PropertyBuyProposal;

#[derive(Accounts)]
#[instruction(property_system_account:Pubkey,proposal_id:u64)]
pub struct BuySubmitSnapshot<'info>{

     #[account(mut,
    address = HARDCODED_PUBKEY  @ ErrorCode::UnAuthorized,
    )]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds=[
            BUYPROPERTY,
            property_system_account.as_ref(),
            &proposal_id.to_le_bytes(),
        ],
        bump = proposal.bump,
        constraint = !proposal.snapshot_submitted @ ErrorCode::SnapshotAlreadySubmitted,
        constraint = proposal.status == ProposalStatus::Approved @ ErrorCode::NotInDraft,
        constraint = proposal.is_arbitrar_approved @ ErrorCode::ProposalNotApproved
    )]
    pub proposal : Account<'info,PropertyBuyProposal>,

}

pub fn buy_submit_snapshot(
    ctx:Context<BuySubmitSnapshot>,
    _property_system_account:Pubkey,
    _proposal_id:u64,
    merkle_root : [u8;32],
    closing_days_gap : u8,
    payment_deadline_days : u8,
    vote_threshold :u64,
)->Result<()>{

    
    require!( 0 < vote_threshold  && vote_threshold< ctx.accounts.proposal.total_voting_power, ErrorCode::InvalidVotingThreshold);

    require!(closing_days_gap <= 30 && closing_days_gap>0,ErrorCode::ClosingDay);

    require!(payment_deadline_days > 0, ErrorCode::PaymentDeadline);

    let proposal = &mut *ctx.accounts.proposal;

    submit(proposal, merkle_root, closing_days_gap,vote_threshold,payment_deadline_days)?;


    

    

    

    

 
    Ok(())


}
