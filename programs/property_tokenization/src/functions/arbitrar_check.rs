    use crate::{constant::Arbitrable, events::SnapshotRequested};
use anchor_lang::prelude::*;
use crate::errors::ErrorCode;

pub fn arbitrar_approval<T: Arbitrable>(
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

        emit!(SnapshotRequested {
            proposal_id: item.proposal_id(),
            mint: governance_mint,
            slot,
        });

    }

    Ok(())
}