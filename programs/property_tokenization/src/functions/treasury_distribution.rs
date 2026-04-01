use anchor_lang::prelude::*;

use anchor_spl::{token_interface::{Mint,TransferChecked, transfer_checked, TokenAccount, TokenInterface}};

use crate::state::TreasuryPda;


pub fn transfer_from_treasury<'info>(
    amount: u64,
    from: &InterfaceAccount<'info, TokenAccount>,
    to: &InterfaceAccount<'info, TokenAccount>,
    mint: &InterfaceAccount<'info, Mint>,
    authority: &Account<'info, TreasuryPda>,
    token_program: &Interface<'info, TokenInterface>,
    signer_seeds: &[&[&[u8]]],
) -> Result<()> {

    
    
    let cpi_accounts = TransferChecked {
        from: from.to_account_info(),
        mint: mint.to_account_info(),
        to: to.to_account_info(),
        authority: authority.to_account_info(),
    };

    let cpi_ctx = CpiContext::new_with_signer(
        token_program.to_account_info(),
        cpi_accounts,
        signer_seeds,
    );


    transfer_checked(cpi_ctx, amount,mint.decimals)?;

    Ok(())

}