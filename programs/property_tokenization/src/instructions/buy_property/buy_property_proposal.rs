use anchor_lang::prelude::*;

use crate::common::{BUYPROPERTY, PROPERTY_SEED, PROPERTY_SYSTEM_SEEDS, ProposalStatus, ProposalType, REINVESTMENTPDA, SELLPROPERTY, TRUSTEE_RECEIPT_SEEDS};
use crate::state::{PropertyAccount, PropertyBuyProposal, PropertySellProposal, PropertySystemAccount, ReinvestmentPda, TrusteeRecepit};
use crate::errors::ErrorCode;


#[derive(Accounts)]
#[instruction(proposal_id : u64,buyer_property_system_id:u64,seller_property_system_account:Pubkey,seller_proposal_id:u64,
    state_pubkey:Pubkey,property_id:u64)]
pub struct BuyPropertyProposal<'info>{

    #[account(
        mut,
        // constraint = trustee_registry.trustees.contains(&trustee.key()) @ ErrorCode::NotAuthorized
    )]
    pub trustee: Signer<'info>,

     #[account(
        seeds = [
            TRUSTEE_RECEIPT_SEEDS,
            buyer.key().as_ref(),
            trustee.key().as_ref()
        ],
        bump = trustee_receipt.bump,
    )]
    pub trustee_receipt: Box<Account<'info,TrusteeRecepit>>,


    #[account(
        seeds =[
                PROPERTY_SYSTEM_SEEDS,
                &buyer_property_system_id.to_le_bytes()
        ],
        bump = buyer.bump,
    )]
    pub buyer:Box<Account<'info,PropertySystemAccount>>,

    // #[account(
    //     constraint = buyer.trustee_registry == trustee_registry.key() @ ErrorCode::InvalidTrusteeRegsitry
    // )]
    // pub trustee_registry: Account<'info,TrusteeRegistry>,

    // #[account(
    //     constraint = buyer.treasury == buyer_treasury.key() @ ErrorCode::InvalidTreasury
    // )]
    // pub buyer_treasury: Account<'info,TreasuryPda>,

    #[account(
        seeds = [
            REINVESTMENTPDA,
            buyer.key().as_ref()
        ],
        bump = buyer_reinvestment_pda.bump,
    )]
    pub buyer_reinvestment_pda:Box<Account<'info,ReinvestmentPda>>,
    
    #[account(
        seeds=[
            SELLPROPERTY,
            seller_property_system_account.as_ref(),
            &seller_proposal_id.to_le_bytes(),  
        ],
        bump = seller_proposal.bump,
        // constraint = seller_proposal.property_account == property.key() @ ErrorCode::InvalidProperty,
        constraint = seller_proposal.proposal_type == ProposalType::SELLPROPERTY,
        constraint = seller_proposal.status == ProposalStatus::Passed @ ErrorCode::PropertyNotPass,
    )]
    pub seller_proposal : Box<Account<'info,PropertySellProposal>>,

    #[account(
        seeds=[ 
            PROPERTY_SEED,
            &property_id.to_le_bytes(),                
            state_pubkey.as_ref(),                
        ],bump = property_account.bump,
        constraint = property_account.property_system == seller_proposal.property_system_account @ ErrorCode::InvalidLandForSource,
        // constraint = property_account.state_pubkey == state_pubkey @ ErrorCode::InvalidLandForSource,
        constraint = !property_account.is_leased @ ErrorCode::LandCurrentlyLeased,
    )]
    pub property_account : Box<Account<'info,PropertyAccount>>,


    #[account(
        init,
        payer = trustee,
        seeds=[
            BUYPROPERTY,
            buyer.key().as_ref(),
            &proposal_id.to_le_bytes()
        ],
        bump,
        space = 8 + PropertyBuyProposal::SIZE
    )]
    pub proposal : Box<Account<'info,PropertyBuyProposal>>,

    pub system_program : Program<'info,System>,

}


pub fn createbuyproposal(
    ctx:Context<BuyPropertyProposal>,
    proposal_id : u64,
    _buyer_property_system_id:u64,
    _seller_property_system_account:Pubkey,
    _seller_proposal_id:u64,
    _state_pubkey:Pubkey,
    _property_id:u64
) ->Result<()>{

    let buy_proposal = &mut ctx.accounts.proposal;
    
    let buyer = & ctx.accounts.buyer;

    let buyer_wallet = & ctx.accounts.buyer_reinvestment_pda;

    let seller_proposal = & ctx.accounts.seller_proposal;


    let current_time = Clock::get()?.unix_timestamp ;

    require!(current_time < seller_proposal.transfer_deadline , ErrorCode::TransferDeadLineReached);
    require!(buyer.key() != seller_proposal.property_system_account,ErrorCode:: SamePropertySystem);

    buy_proposal.initialize(
        proposal_id,
        buyer.key(),
        buyer_wallet.key(),
        seller_proposal.property_account,
        seller_proposal.key(),
        seller_proposal.sale_price,
        ctx.bumps.proposal,
        buyer.total_token_supply,
    );


    Ok(())
}
