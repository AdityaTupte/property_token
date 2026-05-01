use anchor_lang::prelude::*;

use crate::{common::{ARBITRAR_RECEIPT_SEEDS, ARBITRAR_RESIGNATION, AuthorityType, ELECT_ARBITRAR, PROPERTY_SYSTEM_SEEDS, ProposalStatus}, errors::ErrorCode, state::{ArbitratorRecepit, ElectAuthority, PropertySystemAccount, Resignation}};

#[derive(Accounts)]
#[instruction(proposal_id:u64,property_system_id:u64)]
pub struct NewArbitrarElectionProposal<'info>{

    #[account(
        mut,
        // constraint = arbitrar_registry.arbitrator.contains(&arbitrar.key()) @ ErrorCode::NotAuthorized
    )]
    pub arbitrar: Signer<'info>,


    #[account(
        seeds = [
            ARBITRAR_RECEIPT_SEEDS,
            property_system.key().as_ref(),
            arbitrar.key().as_ref()
        ],
        bump = arbitrar_receipt.bump,
    )]
    pub arbitrar_receipt: Account<'info,ArbitratorRecepit>,


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
        payer = arbitrar,
        seeds=[
            ARBITRAR_RESIGNATION,
            property_system.key().as_ref(),
            arbitrar.key().as_ref(),  
        ],
        bump,
        space = 8 + Resignation::SIZE
        // constraint = resignation.status ==  ProposalStatus::Pending @ ErrorCode::AlreadyExecuted
    )]
    pub resignation: Account<'info,Resignation>,

    // #[account(
    //     seeds=[
    //         ARBITRAR_REGISTRYSEEDS,
    //         property_system.key().as_ref()
    //     ],
    //     bump = arbitrar_registry.bump
    // )]
    // pub arbitrar_registry: Account<'info,ArbitratorRegistry>,

    #[account(
        init_if_needed,
        payer = arbitrar,
        seeds=[
            ELECT_ARBITRAR,
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
    ctx:Context<NewArbitrarElectionProposal>,
    proposal_id:u64,

    _property_system_id:u64
)->Result<()>{

    let  proposal = &mut ctx.accounts.proposal;
    
    let resignation = &mut ctx.accounts.resignation;

    if !proposal.is_initialized  {

        proposal.property_system = ctx.accounts.property_system.key();

        proposal.authority_type = AuthorityType::ARBITRATOR;

        proposal.status = crate::common::ProposalStatus::Draft;

        proposal.proposal_id = proposal_id;

        proposal.bump = ctx.bumps.proposal;  

        proposal.is_initialized = true;


    }

    else {
        require!(proposal.status == ProposalStatus::Draft,ErrorCode::NotInDraft);
        
         require!(proposal.arbitrar_approvals_count == 0, ErrorCode::TotalApprovalCountInvalid);

    }

    resignation.authority = ctx.accounts.arbitrar.key();

    resignation.property_system = ctx.accounts.property_system.key();

    resignation.authority_type = AuthorityType::ARBITRATOR;

    resignation.bump = ctx.bumps.resignation;

    resignation.status = ProposalStatus::Pending;

    resignation.proposal = proposal.key();

    proposal.total_authority_to_resign += 1;

    Ok(())
}