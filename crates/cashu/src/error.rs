use std::string::FromUtf8Error;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    /// Parse Url Error
    #[error("`{0}`")]
    UrlParseError(#[from] url::ParseError),
    /// Utf8 parse error
    #[error("`{0}`")]
    Utf8ParseError(#[from] FromUtf8Error),
    /// Serde Json error
    #[error("`{0}`")]
    SerdeJsonError(serde_json::Error),
    /// Base64 error
    #[error("`{0}`")]
    Base64Error(#[from] base64::DecodeError),
    #[error("`{0}`")]
    CustomError(String),
    /// From hex error
    #[error("`{0}`")]
    HexError(#[from] hex::FromHexError),
    #[error("`{0}`")]
    EllipticCurve(#[from] k256::elliptic_curve::Error),
    #[error("No Key for Amoun")]
    AmountKey,
    #[error("Amount miss match")]
    Amount,
    #[error("Token already spent")]
    TokenSpent,
    #[error("Token not verified")]
    TokenNotVerifed,
    #[error("Invoice Amount undefined")]
    InvoiceAmountUndefined,
}

#[cfg(feature = "wallet")]
pub mod wallet {
    use std::string::FromUtf8Error;

    use thiserror::Error;

    #[derive(Debug, Error)]
    pub enum Error {
        /// Serde Json error
        #[error("`{0}`")]
        SerdeJsonError(#[from] serde_json::Error),
        /// From elliptic curve
        #[error("`{0}`")]
        EllipticError(#[from] k256::elliptic_curve::Error),
        /// Insufficient Funds
        #[error("Insufficient funds")]
        InsufficientFunds,
        /// Utf8 parse error
        #[error("`{0}`")]
        Utf8ParseError(#[from] FromUtf8Error),
        /// Base64 error
        #[error("`{0}`")]
        Base64Error(#[from] base64::DecodeError),
        /// Unsupported Token
        #[error("Token unsupported")]
        UnsupportedToken,
        /// Token Requires proofs
        #[error("Proofs Required")]
        ProofsRequired,
        /// Url Parse error
        #[error("Url Parse")]
        UrlParse,
        /// Custom Error message
        #[error("`{0}`")]
        CustomError(String),
    }

    impl From<crate::url::Error> for Error {
        fn from(_err: crate::url::Error) -> Error {
            Error::UrlParse
        }
    }
}

#[cfg(feature = "mint")]
pub mod mint {
    use thiserror::Error;

    #[derive(Debug, Error)]
    pub enum Error {
        #[error("No key for amount")]
        AmountKey,
        #[error("Amount miss match")]
        Amount,
        #[error("Token Already Spent")]
        TokenSpent,
        /// From elliptic curve
        #[error("`{0}`")]
        EllipticError(#[from] k256::elliptic_curve::Error),
        #[error("`Token not verified`")]
        TokenNotVerifed,
        #[error("Invoice amount undefined")]
        InvoiceAmountUndefined,
        /// Duplicate Proofs sent in request
        #[error("Duplicate proofs")]
        DuplicateProofs,
        #[error("`{0}`")]
        CustomError(String),
    }
}
