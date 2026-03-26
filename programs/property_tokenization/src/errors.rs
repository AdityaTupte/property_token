use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode{

    #[msg("the candidate submission is not started yet or deadline is reached")]
    CandidateSubmissionDeadline,

    #[msg("the voting for authority is not started yet or deadline is reached")]
    AuthorityVotingDeadline,

    #[msg("the authority type does not match the requirement type")]
    AuthotityTypeNotMatched,

    #[msg("the voter voting limit reached ")]
    VotingLimitReached,

    #[msg("the time for authority adding is not started yet or deadline is reached")]
    AuthorityAddDeadline,

    #[msg("invalid authority type please recheck it")]
    InvalidAuthorityType,

    #[msg("the challenge_to candidate not present in the new authority")]
    ChallengeToNotInNewAuthority,

    #[msg("the votes are less than the challenge_from")]
    VoteGainedLess,

    #[msg("both the authorities are same ")]
    InvalidChallenge,

    #[msg("authority not found")]
    AuthorityNotFound,

    #[msg("the mint is invalid for the property_system passed ")]
    InvalidMint,

    #[msg("the new authority and authority to resign lengths are not equal")]
    InvalidAuthorityMapping,

    #[msg("the candidate is finalized or the order of finalization is broke check the order")]
    ChangeCandidateFinalization,
    
    #[msg("the deadline had been reached for creating voting proposal")]
    RemovalProposalDeadline,
    /////////

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

    #[msg("the Property is invalid please check the Property ")]
    InvalidProperty,

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

    #[msg("the voting power should be greater than 0 ")]
    VotingPowerInvalid,

    #[msg("the proposal is not active ")]
    ProposalNotActive,

    #[msg("the proposal is not exceuted ")]
    ProposalNotExecuted,

    #[msg("Proposal Already Passed")]
    ProposalAlreadyPassed,

    #[msg("the proposal's voting is live or the prposal is passed ")]
    DeletingProposalInvalid,

    #[msg("the proposal not passed yet")]
    ProposalNotPassed,

    #[msg("the given treasury is invalid")]
    InvalidTreasury,

    #[msg("the transfer window is closed")]
    TransferWindowClose,

    #[msg("insufficent balance")]
    InsufficentBalance,

    #[msg("can't find land account")]
    LandAccountNotFound,

    #[msg("the land_page does not have any space create new page or give another page")]
    NotEnoughSpace,

    #[msg("the given land page in invalid")]
    LandPageInvalid,

    #[msg("the transfer deadline be more than 0")]
    TransferDeadline,

    #[msg("the reinvestment treasury is invalid")]
    InvalidReinvestAccount,

    #[msg("closing days should be greater than 0 days")]
    ClosingDay,

    #[msg("the proposal should be draft stage")]
    NotInDraft,

    #[msg("proposal is cannot Finalize")]
    AlreadyFinalized,

    #[msg("proposal is already excecuted")]
    AlreadyExecuted,

    #[msg("voting is still live")]
    VotingStillActive,

    #[msg("the threshold should be less than total voting power")]
    InvalidVotingThreshold,

    #[msg("the snapshot is submitted you cant delete proposal now")]
    AlreadyActivated,

    #[msg("proposal does not achive voting threshold ")]
    ThresholdReject,

    #[msg("deletion of proposal not allowed")]
    DeleteNotAllowed,

    #[msg("the propperty proposal must have status of passed")]
    PropertyNotPass,

    #[msg("the sell property proposal reached the transfer deadline")]
    TransferDeadLineReached,   

    #[msg("the page is full please create a new to add property")]
    InsufficentSpace, 

    #[msg("the payement por tranfer deadline is reached .")]
    CantTramnsfer,

    #[msg("sell proposal key doies niot mnatchjes with the sell proposal in buy proposal")]
    InvalidSellProposal,

    #[msg("the purchasing ans the selling price must be diffrent")]
    DiffrentPrice,
    
    #[msg("the given safety is invalid")]
    InvalidSafety,

    #[msg("threshold must be 100 after adding all threshold")]
    InvalidThreshold,

    #[msg("the deadline for  threshold submission is reached")]
    ThresholdSubmissionEnd,

    #[msg("governance token invaild")]
    GovernanceTokenInvalid,

    #[msg("the signer must hold the token for proposing new threshold")]
    TokenAreZero,

    #[msg("the challenge deadline had been reached")]
    ChallegeDeadlineExpired,
     
    #[msg("the change deadline had been reached")]
    ChangeDeadlineExpired,

    #[msg("the challenge deadline not reached yet")]
    ChallegeDeadlineNotExpired,
    
    #[msg("the deadline should be less than 30")]
    DeadlineIssue,

    #[msg("authority limit reached")]
    AuhtorityLimitReached,

    #[msg("the candidate is not verfied")]
    NotVerfied,

    #[msg("the candidate is blacklisted")]
    Blacklisted,

}