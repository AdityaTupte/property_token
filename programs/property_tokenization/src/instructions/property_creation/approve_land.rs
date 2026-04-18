use anchor_lang::prelude::*;

use crate::common::{PROPERTY_APPROVAL_RECEIPT, PROPERTY_PROPOSAL_SEEDS, STATE_AUTHORITY, STATE_SEEDS};
use crate::state::{PropertyProposal, PropertyProposalReceipt, State, StateAuthority};

use crate::errors::ErrorCode::{self};


#[derive(Accounts)]
#[instruction(country_key:Pubkey,state_name:[u8;32],property_id:u64,)]
pub struct ApproveLand<'info>{

    #[account(
        seeds=[
            STATE_SEEDS,
            state_name.as_ref(),
            country_key.as_ref(),
        ],
        bump = state.bump,
       constraint =  country_key == state.country_pubkey @ ErrorCode::InvalidCountry
    )]
    pub state: Account<'info,State>,


    #[account(
        mut,
        seeds=[
            PROPERTY_PROPOSAL_SEEDS,
            &property_id.to_le_bytes(),
            state.key().as_ref(),
            ],
        bump = property_proposal.bump,
        constraint = !property_proposal.approved @ ErrorCode::AlreadyApproved,
        constraint = property_proposal.state_pubkey == state.key() @ ErrorCode::InvalidProperty
    )]
    pub property_proposal : Account<'info,PropertyProposal>,

    #[account(
        mut,
    )]
    pub signer : Signer<'info>,

     #[account(
        seeds=[
            STATE_AUTHORITY,
            country_key.as_ref(),
            signer.key().as_ref()
        ],
        bump= state_authority_receipt.bump,
    )]
    pub state_authority_receipt : Account<'info,StateAuthority>,


    #[account(
        init,
        payer = signer,
        seeds=[
            PROPERTY_APPROVAL_RECEIPT,
            property_proposal.key().as_ref(),
            signer.key().as_ref()
        ],
        bump,
        space = 8 + PropertyProposalReceipt::SIZE,
    )]
    pub property_approval_receipt : Account<'info,PropertyProposalReceipt>,

    pub system_program: Program<'info,System>,

}

    pub fn approve(
        ctx:Context<ApproveLand>,
        _country_key:Pubkey,_state_name:[u8;32],_property_id:u64,
    )->Result<()>{

        let proposal = &mut ctx.accounts.property_proposal ;

        require!(proposal.approval_count < ctx.accounts.state.threshold,ErrorCode::AuthorityApproved);

        proposal.approval_count += 1 ;

        if proposal.approval_count >= ctx.accounts.state.threshold {

            proposal.approved = true;

        }

        Ok(())
        
    }
