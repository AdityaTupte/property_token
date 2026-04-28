use anchor_lang::prelude::*;

use crate::{common::{AuthorityType, ELECT_TRUSTEE, PROPERTY_SYSTEM_SEEDS, ProposalStatus, TRUSTEE_RECEIPT_SEEDS, TRUSTEE_RESIGNATION}, errors::ErrorCode, state::{ElectAuthority, PropertySystemAccount, Resignation, TrusteeRecepit}};

#[derive(Accounts)]
#[instruction(proposal_id:u64,property_system_id:u64)]
pub struct NewTrusteeElectionProposal<'info>{

    #[account(
        mut,
        // constraint = trustee_registry.trustees.contains(&trustee.key()) @ ErrorCode::NotAuthorized
    )]
    pub trustee: Signer<'info>,

    #[account(
        seeds = [
            TRUSTEE_RECEIPT_SEEDS,
            property_system.key().as_ref(),
            trustee.key().as_ref()
        ],
        bump = trustee_receipt.bump,
    )]
    pub trustee_receipt: Account<'info,TrusteeRecepit>,

    #[account(
        seeds=[
            PROPERTY_SYSTEM_SEEDS,
            &property_system_id.to_le_bytes(),
        ],
        bump = property_system.bump
    )]
    pub property_system : Account<'info,PropertySystemAccount>, 

     #[account(
        init,
        payer = trustee,
        seeds=[
            TRUSTEE_RESIGNATION,
            property_system.key().as_ref(),
            trustee.key().as_ref(),  
        ],
        bump,
        space = 8 + Resignation::SIZE
        // constraint = resignation.status ==  ProposalStatus::Pending @ ErrorCode::AlreadyExecuted
    )]
    pub resignation: Account<'info,Resignation>,

    


    // #[account(
    //     seeds=[
    //         TRUSTEEREGISTRYSEEDS,
    //         property_system.key().as_ref()
    //     ],
    //     bump = trustee_registry.bump
    // )]
    // pub trustee_registry: Account<'info,TrusteeRegistry>,

    #[account(
        init_if_needed,
        payer = trustee,
        seeds=[
            ELECT_TRUSTEE,
            property_system.key().as_ref(),
            &proposal_id.to_le_bytes(),
        ],
        bump ,
        space = 8 + ElectAuthority::SIZE,
    )]
    pub proposal : Account<'info,ElectAuthority>,

    pub system_program:Program<'info,System>,

} 

pub fn new_trustee_election_proposal(
    ctx:Context<NewTrusteeElectionProposal>,
    proposal_id:u64,
    _property_system_id:u64
)->Result<()>{

    let  proposal = &mut ctx.accounts.proposal;
    
    let resignation = &mut ctx.accounts.resignation;

    if !proposal.is_initialized  {

        proposal.property_system = ctx.accounts.property_system.key();

        proposal.authority_type = AuthorityType::TRUSTEE;

        proposal.status = crate::common::ProposalStatus::Draft;

        proposal.proposal_id = proposal_id;

        proposal.bump = ctx.bumps.proposal;  

        proposal.is_initialized = true;


    }

    else {
        require!(proposal.status == ProposalStatus::Draft,ErrorCode::NotInDraft);

        require!(proposal.arbitrar_approvals_count == 0, ErrorCode::TotalApprovalCountInvalid);
    } 

      

    resignation.authority = ctx.accounts.trustee.key();

    resignation.property_system = ctx.accounts.property_system.key();

    resignation.authority_type = AuthorityType::TRUSTEE;

    resignation.bump = ctx.bumps.resignation;

    resignation.status = ProposalStatus::Pending;

    resignation.proposal = proposal.key();

    proposal.total_authority_to_resign += 1;




    // let trustee_key = ctx.accounts.trustee.key();

    // require!(
    //     !proposal.authority_to_resign.contains(&trustee_key),
    //     ErrorCode::DuplicateAuthority
    // );

    // require!(
    //     !proposal.authority_to_resign.len() <= 5 as usize ,ErrorCode::AuthorityLimitReached
    // );

    // proposal.authority_to_resign.push(trustee_key);

    

    Ok(())
}