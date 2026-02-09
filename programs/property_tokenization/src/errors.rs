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

    #[msg(" authorioty pubkey must be 10")]
    ApproveAuthorityInvalid,

    #[msg("the thresold must between 0 to  10")]
    ApproveAuthorityThresholdInvalid,

    #[msg("the signer is nopt authorized to sign the trasaction")]
    NotAuthorized,

    #[msg("the proposal already approved ")]
    AlreadyApproved,

    #[msg("the given  suthotiry already approved proposal  ")]
    AuthorityApproved,
    
    #[msg("country name must be something")]
    CountryNameInvalid,

    #[msg("the give authority has some duplicate authority")]
    DuplicateAuthority,
}