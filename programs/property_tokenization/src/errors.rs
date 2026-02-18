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
    
    #[msg("country name must be between 0 to 32 as uszie")]
    CountryNameInvalid,

    #[msg("state name must be between 0 to 32 as uszie")]
    StateNameInvalid,

    #[msg("the give authorities vector has some duplicate authority")]
    DuplicateAuthority,

    #[msg("the proposal is not approved yet by the authorities")]
    ProposalNotApproved,

    #[msg("the threshold must be between 1  to  10")]
    CountryPdaThresholdInvalid,

    #[msg("the state authority must be  10")]
    StateAuthorityInvalid,
    
    #[msg("the threshold must be in between 1 to 10")]
    StateThresholdInvalid,

    #[msg("the country is invalid please check the country ")]
    InvalidCountry,

    #[msg("the land is invalid please check the land ")]
    InvalidLand,

    #[msg("the trusteeRegistry is Invalid")]
    InvalidTrusteeRegsitry,

    #[msg("the the land does not belong to source property_system")]
    InvalidLandForSource,

    #[msg("create new page the exisiting page is full")]
    PageFull,

    #[msg("the property system is invalid")]
    PropertySystemInvalid,

    #[msg("the source and destination propoerty system are same")]
    SamePropertySystem,

    #[msg("the arbitrar registry  not valid for the given property system")]
    ARBITRARREGISTRYINVALID,

    #[msg("the property system is not valid for given arbitrar registry")]
    PropertySystemInvalidForRegistry,

    #[msg("the snaphot is already submitted")]
    SnapshotAlreadySubmitted,

    #[msg("math overflow")]
    MathOverflow,

    #[msg("the snaphot is not submitted to the proposal")]
    SnapshotNotSubmitted,

    #[msg("the property system is not valid to proposal")]
    InvalidProposal,

    #[msg("the given proof is not valid ")]
    InvalidMerkleProof,

    #[msg("the voting period it not start yet or expired")]
    VotingPeriodExpired,

    #[msg("the voting power exceed the total votes")]
    VotingPowerInvalid,

    #[msg("the proposal is not active ")]
    ProposalNotActive,

    #[msg("Proposal Already Passed")]
    ProposalAlreadyPassed,

    #[msg("the proposal's voting is live or the prposal is passed ")]
    DeletingProposalInvalid,

    


}