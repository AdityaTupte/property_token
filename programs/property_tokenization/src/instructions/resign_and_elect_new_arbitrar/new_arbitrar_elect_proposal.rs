use anchor_lang::prelude::*;

use crate::{common::{ARBITRAR_REGISTRYSEEDS, ARBITRAR_RESIGNATION, AuthorityType, ELECT_ARBITRAR,PROPERTY_SYSTEM_SEEDS, ProposalStatus}, errors::ErrorCode, state::{ArbitratorRegistry, ElectAuthority, PropertySystemAccount, Resignation}};

#[derive(Accounts)]
#[instruction(proposal_id:u64)]
pub struct NewArbitrarElectionProposal<'info>{

    #[account(
        mut,
        constraint = arbitrar_registry.arbitrator.contains(&arbitrar.key()) @ ErrorCode::NotAuthorized
    )]
    pub arbitrar: Signer<'info>,

     #[account(
        seeds=[
            ARBITRAR_RESIGNATION,
            arbitrar.key().as_ref(),
            property_system.key().as_ref(),
        ],
        bump = resignation.bump,
        constraint = resignation.status ==  ProposalStatus::Pending @ ErrorCode::AlreadyExecuted
    )]
    pub resignation: Account<'info,Resignation>,

    #[account(
        seeds=[
            PROPERTY_SYSTEM_SEEDS,
            &property_system.property_system_id.to_le_bytes(),
        ],
        bump = property_system.bump
    )]
    pub property_system : Account<'info,PropertySystemAccount>, 


    #[account(
        seeds=[
            ARBITRAR_REGISTRYSEEDS,
            property_system.key().as_ref()
        ],
        bump = arbitrar_registry.bump
    )]
    pub arbitrar_registry: Account<'info,ArbitratorRegistry>,

    #[account(
        init_if_needed,
        payer = arbitrar,
        seeds=[
            ELECT_ARBITRAR,
            &proposal_id.to_le_bytes(),
            property_system.key().as_ref()
        ],
        bump ,
        space = 8 + ElectAuthority::SIZE,
    )]
    pub proposal : Account<'info,ElectAuthority>,

    pub system_program:Program<'info,System>,

} 

pub fn new_trustee_election_proposal(
    ctx:Context<NewArbitrarElectionProposal>,
    proposal_id:u64


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
    }

    let arbitrar_key = ctx.accounts.arbitrar.key();

    require!(
        !proposal.authority_to_resign.contains(&arbitrar_key),
        ErrorCode::DuplicateAuthority
    );

    require!(
        !proposal.authority_to_resign.len() <= 5 as usize ,ErrorCode::AuhtorityLimitReached
    );

    proposal.authority_to_resign.push(arbitrar_key);

    resignation.proposal = proposal.key();

    Ok(())
}