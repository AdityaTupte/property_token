use crate::{common::ProposalStatus, traits::AuthorityGovernance, events::{ SnapshotRequestedForAuthority}, state::{ VoteReceiptForAuthorityElection}};
use anchor_lang::prelude::*;


pub fn arbitrar_approval_for_authority<T: AuthorityGovernance>(
    proposal:&mut T,
    registry_vote_threshold : u8,
    voter_receipt: &mut Account<VoteReceiptForAuthorityElection>,
    signer:Pubkey,
    proposal_key:Pubkey,
    governance_mint:Pubkey,
    property_system_account:Pubkey
)->Result<()>{

    // let approvals = item.arbitrar_list();

    // require!(
    //     !approvals.contains(&signer),
    //     ErrorCode::AuthorityApproved
    // );

    // approvals.push(signer);

    // if approvals.len() >= 3 {

    //     // *item.arbitrar_approved() = true;

    //     // *item.slot()= Clock::get()?.slot;

    //     // *item.proposal_status() = ProposalStatus::Active;

    //     emit!(SnapshotRequestedForAuthority {
    //         proposal_id: *item.proposal_id(),
    //         proposal_type : *item.proposal_type(),
    //         proposal_key: proposal_key,
    //         mint: governance_mint,
    //         slot:*item.slot(),
    //     });

    // }

    *proposal.arbitrar_total_count() += 1;

    voter_receipt.proposal_key = proposal_key;

    voter_receipt.property_system = property_system_account;

    voter_receipt.authority_key = signer;
    
    voter_receipt.authority_type = *proposal.proposal_type();


    if *proposal.arbitrar_total_count() >= registry_vote_threshold {

        *proposal.arbitrar_approved() = true;

        *proposal.proposal_status() = ProposalStatus::Approved;

        let snapshot_slot= Clock::get()?.slot;

        *proposal.slot() =snapshot_slot;

        emit!(SnapshotRequestedForAuthority {
            proposal_id: *proposal.proposal_id(),
            proposal_type:*proposal.proposal_type(),
            proposal_key: proposal_key,
            mint: governance_mint,
            slot: snapshot_slot,
        });


    }


    Ok(())
}