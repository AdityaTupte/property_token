use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode{

    #[msg("the property system counter is Invalid please check that the property system counter is correct")]
    PropertyCounterInvalid,

    #[msg("the token miniting to the creator account is failed due to some reason")]
    MintFailed,

    #[msg("the stable coin mint is in invalid please check that the mint is correct")]
    StableMintInvalid,

    #[msg("the threshold sum is not equal 100 ")]
    ThresholdInvalid,

}