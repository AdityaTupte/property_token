use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;

declare_id!("8PRrAQa9Y4ZVxcdNXP4Q8CiQzAyiWoyJT64t7FUmrN4L");

#[program]
pub mod peoperty_tokenization {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
