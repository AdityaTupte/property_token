use anchor_lang::prelude::*;

#[account]
pub struct SafetyPda{

    pub property_system: Pubkey,

    pub bump: u8,


}

impl SafetyPda {

    pub const SIZE:usize = 32 + 1;
    
}