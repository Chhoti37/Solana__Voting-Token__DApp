use anchor_lang::prelude::*;
#[error_code]
pub enum VoteError {
    #[msg("invalid deadline")]
    InvalidDeadline,

    #[msg("Proposal counter already initialized")]
    ProposalCounterAlreadyInitialized,

    #[msg("Proposal Counter Overflow")]
    ProposalCounterOverflow,

    #[msg("Proposal Ended")]
    ProposalEnded,

    #[msg("Proposal Ended")]
    ProposalVotesOverflow,

    #[msg("Voting is still active - cannot declare winner yet")]
    VotingStillActive,

    #[msg("No votes cast for this proposal")]
    NoVotesCast,

    #[msg("Unauthorized access")]
    UnauthorizedAccess,
    
    #[msg("Token mint mismatch")]
    TokenMintMismatch,

    #[msg("Voter has already voted on this proposal")]
    VoterAlreadyVoted,

    #[msg("Token account is not owned by the expected wallet")]
    InvalidTokenAccountOwner,

    #[msg("Provided mint account is invalid")]
    InvalidMint,


}

