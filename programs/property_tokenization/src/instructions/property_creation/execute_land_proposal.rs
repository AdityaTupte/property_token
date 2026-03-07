    use anchor_lang::prelude::*;

    use crate::common::{PROPERTY_METADATA_SEEDS, PROPERTY_PAGE_SEEDS, PROPERTY_PROPOSAL_SEEDS, PROPERTY_SEED, PROPERTY_SYSTEM_SEEDS, STATE_SEEDS};
    use crate::state::{ PropertyAccount, PropertyAccountMetadata, PropertyPage, PropertyProposal, PropertySystemAccount, State};

    use crate::errors::ErrorCode::{self};

    #[derive(Accounts)]

    pub struct ExecutePropertyProposal<'info>{

        #[account(
            mut,
            seeds = [ 
                PROPERTY_SYSTEM_SEEDS,
                &property_system_account.property_system_id.to_le_bytes(),
            ],
            bump,
        )]
        pub property_system_account : Account<'info,PropertySystemAccount>,


        #[account(
            mut,
            seeds = [
                PROPERTY_PAGE_SEEDS,
                &property_page.page.to_le_bytes(),
                property_system_account.key().as_ref()
            ],
            bump = property_page.bump,
            constraint = property_page.property_system == property_system_account.key() @ ErrorCode::PropertySystemInvalid
        )]

        pub property_page : Account<'info,PropertyPage>,


        #[account(
            seeds = [
                STATE_SEEDS,
                &state.state_id.to_le_bytes(),
                state.country_pubkey.as_ref(),
            ],
            bump = state.bump,
        )]
        pub state: Account<'info,State>,

        #[account(
            mut,
            seeds=[
                PROPERTY_PROPOSAL_SEEDS,
                &property_proposal.property_id.to_le_bytes(),
                state.key().as_ref(),
            ],
            bump = property_proposal.bump,
            constraint = property_proposal.approved @ ErrorCode::ProposalNotApproved,
            constraint = property_proposal.state_pubkey == state.key() @ ErrorCode::InvalidProperty,
            close = signer
        )]

        pub property_proposal: Account<'info,PropertyProposal>,

        #[account(
            mut,
            constraint = state.authorities.contains(&signer.key()) @ ErrorCode::NotAuthorized
        )]

        pub signer: Signer<'info>,

        #[account(
            init,
            payer = signer,
            seeds = [
                    PROPERTY_SEED,
                    &property_proposal.property_id.to_le_bytes(),
                    state.key().as_ref(),
            ],
            bump,
            space = 8 + PropertyAccount::SIZE
        )]

        pub property_pda : Account<'info,PropertyAccount>,

        #[account(
            init,
            payer = signer,
            seeds = [
                PROPERTY_METADATA_SEEDS,
                property_pda.key().as_ref(),    
            ],
            bump,
            space = 8 + PropertyAccountMetadata::SIZE
        )]
        pub property_metadata : Account<'info,PropertyAccountMetadata>,

        pub system_program : Program<'info,System>,

    }

    pub fn execute(
        ctx:Context<ExecutePropertyProposal>
    )->Result<()>{

        let proposal = &mut ctx.accounts.property_proposal ;

        let property_acc = &mut ctx.accounts.property_pda;

        let metadata = &mut ctx.accounts.property_metadata;

        let property_page  = &mut ctx.accounts.property_page;

        let state = & ctx.accounts.state;

        let current_time = Clock::get()?.unix_timestamp;
        
        require!(property_page.land.len() < 100 , ErrorCode::PageFull);
        require!(!proposal.executed, ErrorCode::AlreadyExecuted);

        property_page.land.push(property_acc.key());

        property_acc.property_id = proposal.property_id;

        property_acc.property_system = ctx.accounts.property_system_account.key();

        property_acc.page_number = property_page.page;
    
        property_acc.state_id = state.state_id;

        property_acc.state_pubkey = state.key();

        property_acc.country_id  = state.country_id;

        property_acc.country_pubkey = state.country_pubkey;

        property_acc.issued_at  = current_time;

        property_acc.issued_by = proposal.issued_by;

        property_acc.metadata = metadata.key();

        property_acc.bump = ctx.bumps.property_pda;
    
        metadata.property = property_acc.key();

        metadata.legal_doc_hash = proposal.legal_doc_hash;
        
        metadata.last_updated = current_time;

        metadata.bump = ctx.bumps.property_metadata;

        proposal.executed = true;
        
        Ok(())
    }