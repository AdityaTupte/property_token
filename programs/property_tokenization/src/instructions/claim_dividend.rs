use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken,  token_interface::{Mint, TokenAccount,transfer_checked, TokenInterface, TransferChecked}};

use crate::{common::DIVIDENDSEEDS, state::{DividendPda}, transfer_hook::accounts::RewardPda};




#[derive(Accounts)]
pub struct ClaimDividend<'info>{

    #[account(
        mut,
    )]
    pub signer : Signer<'info>,
    
     #[account(
        associated_token::mint = governance_mint,
        associated_token::authority = signer,
        associated_token::token_program = token_program,
    )]
    pub signer_governance_mint_ata : InterfaceAccount<'info,TokenAccount>,

    
    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = mint,
        associated_token::authority = signer,
        associated_token::token_program = token_program,
    )]
    pub signer_ata : InterfaceAccount<'info,TokenAccount>,

     #[account(
        mut,
        seeds=[
            b"dividend",
            governance_mint.key().as_ref(),
        ],
        bump = dividend_pda.bump)]
    pub dividend_pda : Account<'info,DividendPda>,

    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = mint,
        associated_token::authority = dividend_pda,
        associated_token::token_program = token_program,
    )]
    pub dividend_ata : Box<InterfaceAccount<'info,TokenAccount>>,


    #[account(
        seeds=[
            b"rewardpda",
            governance_mint.key().as_ref(),
            signer_governance_mint_ata.key().as_ref(),
        ],
        bump,
        seeds::program = transfer_hook_program.key(),
    )]
    pub reward_pda : Account<'info,RewardPda>,

    pub transfer_hook_program: Program<'info, crate::transfer_hook::program::TransferHook>,

    pub mint : Box<InterfaceAccount<'info,Mint>>,

    pub governance_mint :  Box<InterfaceAccount<'info,Mint>>,

    pub associated_token_program: Program<'info, AssociatedToken>,

    pub token_program : Interface<'info,TokenInterface>,

    pub system_program :Program<'info,System>,

}


pub fn claim_dividend(
    ctx:Context<ClaimDividend>,
)->Result<()>{

    let dividend_pda = & ctx.accounts.dividend_pda;

    let cpi_accounts = TransferChecked{
        from:ctx.accounts.dividend_ata.to_account_info(),
        to:ctx.accounts.signer_ata.to_account_info(),
        mint: ctx.accounts.mint.to_account_info(),
        authority:ctx.accounts.dividend_pda.to_account_info()
    };

    let g_key =  & ctx.accounts.governance_mint.key();
    
    let signer_seeds:&[&[&[u8]]] = &[&[
        DIVIDENDSEEDS,
        g_key.as_ref(),
        &[dividend_pda.bump]
    ]];


    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        cpi_accounts, 
        signer_seeds
    );

    transfer_checked(cpi_ctx, ctx.accounts.reward_pda.pending_reward, ctx.accounts.mint.decimals)?;

    Ok(())
}