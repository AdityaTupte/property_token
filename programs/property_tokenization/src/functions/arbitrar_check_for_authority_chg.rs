use crate::{common::{ ProposalStatus}, constant::AuthorityGovernance, events::{ SnapshotRequestedForAuthority}};
use anchor_lang::prelude::*;
use crate::errors::ErrorCode;

pub fn arbitrar_approval_for_authority<T: AuthorityGovernance>(
    item:&mut T,
    signer:Pubkey,
    proposal_key:Pubkey,
    governance_mint:Pubkey,
)->Result<()>{

    let approvals = item.arbitrar_list();

    require!(
        !approvals.contains(&signer),
        ErrorCode::AuthorityApproved
    );

    approvals.push(signer);

    if approvals.len() >= 3 {

        *item.arbitrar_approved() = true;

        *item.slot()= Clock::get()?.slot;

        *item.proposal_status() = ProposalStatus::Active;

        emit!(SnapshotRequestedForAuthority {
            proposal_id: *item.proposal_id(),
            proposal_type : *item.proposal_type(),
            proposal_key: proposal_key,
            mint: governance_mint,
            slot:*item.slot(),
        });

    }

    Ok(())
}