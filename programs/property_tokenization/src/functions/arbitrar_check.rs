    use crate::{constant::Governance, events::SnapshotRequested};
use anchor_lang::prelude::*;
use crate::errors::ErrorCode;

pub fn arbitrar_approval<T: Governance>(
    item:&mut T,
    signer:Pubkey,
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

        let slot = Clock::get()?.slot;

        *item.slot() = slot;

        emit!(SnapshotRequested {
            proposal_id: *item.proposal_id(),
            mint: governance_mint,
            slot,
        });


    }

    Ok(())
}