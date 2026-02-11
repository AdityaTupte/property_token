use anchor_lang::prelude::*;

use crate::state::{LandAccount, LandAccountMetadata, State};
use crate::events::*;


const LAND_SEED: &[u8] = b"land_account";

#[derive(Accounts)]
#[instruction(land_id:u16)]
pub struct CreateLandAccount<'info>{

    #[account(mut)]

    pub signer: Signer<'info>,

    #[account(
        seeds = [
        b"state",
        state.state_id.to_be_bytes().as_ref(),
        state.country_id.to_le_bytes().as_ref()
    ],
    bump = state.bump,
    constraint  = state.authorities.contains(&signer.key())
    )]

    pub state: Account<'info,State>,


    #[account(
        init,
        payer = signer,
        seeds = [
            LAND_SEED,
            state.state_id.to_le_bytes().as_ref(),
            land_id.to_le_bytes().as_ref()
        ],
        bump,
        space = 8 + LandAccount::SIZE
    )]

    pub land_acc: Account<'info,LandAccount>,

    #[account(
        init,
        payer = signer,
        seeds = [
            b"land_metadata",
            land_acc.key().as_ref(),
        ],
        bump,
        space = 8 + LandAccountMetadata::SIZE

    )]

    pub land_metadata: Account<'info,LandAccountMetadata>,

    pub system_program : Program<'info,System>,
     

}

pub fn create_land_acc(
    ctx:Context<CreateLandAccount>,
    land_id :u16,
    legal_doc_hash : [u8;32]
)->Result<()>{

    
    let land_acc =  &mut ctx.accounts.land_acc;

    let land_acc_metadata = &mut ctx.accounts.land_metadata;
    
    let state =  &ctx.accounts.state;

    let current_time = Clock::get()?.unix_timestamp;
    
    //land_account details

    land_acc.land_id = land_id;
    
    land_acc.state_id = state.state_id;

    land_acc.country_id = state.country_id;

    land_acc.issued_at = current_time;

    land_acc.issued_by = ctx.accounts.signer.key();

    land_acc.metadata = land_acc_metadata.key();

    land_acc.bump = ctx.bumps.land_acc;

    //land_metada details

    land_acc_metadata.land = land_acc.key();

    land_acc_metadata.legal_doc_hash = legal_doc_hash;

    land_acc_metadata.last_updated = current_time;

    land_acc_metadata.bump = ctx.bumps.land_metadata;


    emit!(LandAccountCreated{

        land_account : land_acc.key(),

        state : state.key(),
    
        issued_by : ctx.accounts.signer.key(),

        land_metadata: land_acc_metadata.key(),
    });

    Ok(())


}