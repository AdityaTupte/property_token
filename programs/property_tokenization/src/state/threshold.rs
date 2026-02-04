use anchor_lang::prelude::*;

#[account]

pub struct Threshold{

    pub trustee_salary_threshold : u8,

    pub arbitrator_salary_threshold : u8,

    pub dividend_threshold: u8,

    pub reinvestment_threshold : u8,

    pub safety_threshold : u8,

}


impl Threshold {

    pub const SIZE:usize= 1 + 1 + 1 + 1 + 1;
    
}
