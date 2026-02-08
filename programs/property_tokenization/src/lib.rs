use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;
pub mod events;
pub mod errors;

use crate::instructions::*;

declare_id!("8PRrAQa9Y4ZVxcdNXP4Q8CiQzAyiWoyJT64t7FUmrN4L");

#[program]
pub mod peoperty_tokenization {

    use super::*;

    pub fn create_property_system_account(
        ctx:Context<PropertySystemAcc>,
        decimal:u8,
        amount:u64,
        safety_threshold:u8,
        trustee_salary_threshold:u8,
        arbitrator_salary_threshold:u8,
        dividend_threshold:u8,
        reinvestment_threshold:u8
    ) -> Result<()> {
        
        create_property_system_account::create(
            ctx,
            decimal,
            amount, 
            safety_threshold, 
            trustee_salary_threshold, 
            arbitrator_salary_threshold, 
            dividend_threshold, 
            reinvestment_threshold
        )?;


        Ok(())
    }

    pub fn create_land_account(
        ctx:Context<CreateLandAccount>,
        land_id :u16,
        legal_doc_hash : [u8;32]
    )->Result<()>{

        create_land_acc::create_land_acc(ctx, land_id, legal_doc_hash)?;
        Ok(())


}

}
