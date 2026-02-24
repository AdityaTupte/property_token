use anchor_lang::prelude::*;
use crate::constant::*;
use crate::errors::ErrorCode;
use crate::functions::submit;
use crate::state::TransferLandDetail;

#[derive(Accounts)]

pub struct SubmitSnapshot<'info>{

    #[account(
        constraint = proposal.arbitrar_approval.contains(&signer.key()),
        constraint = proposal.arbitrar_approved
    )]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds=[
            SELLPROPERTY,
            proposal.seller.as_ref(),
            &proposal.proposal_id.to_le_bytes(),
        ],
        bump = proposal.bump,
        constraint = !proposal.snapshot_submitted @ ErrorCode::SnapshotAlreadySubmitted
    )]

    pub proposal : Account<'info,TransferLandDetail>,

}

pub fn submit_snapshot(
    ctx:Context<SubmitSnapshot>,
    merkle_root : [u8;32],
    closing_days_gap : u8,
)->Result<()>{

    let proposal = &mut *ctx.accounts.proposal;

    require!(closing_days_gap>0,ErrorCode::ClosingDay);

    submit(proposal, merkle_root, closing_days_gap)?;
 
    Ok(())


}
