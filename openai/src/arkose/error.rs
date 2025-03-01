#[derive(thiserror::Error, Debug)]
pub enum ArkoseError {
    #[error("submit funcaptcha answer error {0:?}")]
    SubmitAnswerError(anyhow::Error),
    #[error("Invalid arkose platform type: {0:?}")]
    InvalidPlatformType(String),
    #[error("Invalid GPT model: {0:?}")]
    InvalidGptModel(String),
    #[error("No solver available or solver is invalid")]
    NoSolverAvailable,
    #[error("Error creating arkose session error {0:?}")]
    CreateSessionError(anyhow::Error),
    #[error("invalid funcaptcha error")]
    InvalidFunCaptcha,
    #[error("hex decode error")]
    HexDecodeError,
    #[error("unsupported hash algorithm")]
    UnsupportedHashAlgorithm,
}
