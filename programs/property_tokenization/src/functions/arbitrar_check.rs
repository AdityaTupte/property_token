use crate::{common::ProposalStatus, constant::Governance, events::SnapshotRequested, state::{ArbitratorRegistry, ArbitratorVoteReceipts}};
use anchor_lang::{prelude::*};


// pub fn arbitrar_approval<T: Governance>(
//     item:&mut T,
//     signer:Pubkey,
//     proposal_key:Pubkey,
//     governance_mint:Pubkey,
// )->Result<()>{

//     let approvals = item.arbitrar_list();

//     require!(
//         !approvals.contains(&signer),
//         ErrorCode::AuthorityApproved
//     );

//     approvals.push(signer);

//     if approvals.len() >= 3 {

//         *item.arbitrar_approved() = true;

//         emit!(SnapshotRequested {
//             proposal_id: *item.proposal_id(),
//             proposal_key: proposal_key,
//             mint: governance_mint,
//             slot:*item.slot() ,
//         });


//     }

//     Ok(())
// }



pub fn arbitrar_approval<T:Governance>(
    proposal: &mut T,
    proposal_key:Pubkey,
    arbitrar_registry : &Account<ArbitratorRegistry>,
    arbitrar_voter: &mut Account<ArbitratorVoteReceipts>,
    signer:Pubkey,
    governance_mint:Pubkey,
    property_system_account:Pubkey
)->Result<()>{

    *proposal.arbitrar_total_count() += 1;

    arbitrar_voter.proposal_key = proposal_key;

    arbitrar_voter.property_system_key = property_system_account;

    arbitrar_voter.arbitrator_key = signer;
    
    arbitrar_voter.proposal_type = *proposal.proposal_type();


    if *proposal.arbitrar_total_count() >= arbitrar_registry.vote_threshold {

        *proposal.arbitrar_approved() = true;

        *proposal.proposal_status() = ProposalStatus::Approved;

        let snapshot_slot= Clock::get()?.slot;

        *proposal.slot() =snapshot_slot;

        emit!(SnapshotRequested {
            proposal_id: *proposal.proposal_id(),
            proposal_key: proposal_key,
            mint: governance_mint,
            slot: snapshot_slot,
        });


    }

    Ok(())
}
