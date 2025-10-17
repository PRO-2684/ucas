//! # `ucas-iclass` library crate
//!
//! If you are reading this, you are reading the documentation for the `ucas-iclass` library crate. For the cli, kindly refer to the README file.

#![deny(missing_docs)]
#![warn(clippy::all, clippy::nursery, clippy::pedantic, clippy::cargo)]

mod login;

pub use login::LoginResult;
use cyper::{Client, Error as CyperError};
use serde::Deserialize;

/// The root URL of the iClass platform.
pub const API_ROOT: &str = "https://iclass.ucas.edu.cn:8181/";

/// The iClass struct.
pub struct IClass {
    /// The HTTP client.
    client: Client,
    /// Login result.
    login_result: Option<LoginResult>,
}

/// Possible errors when interacting with the iClass platform.
#[derive(Debug, thiserror::Error)]
pub enum IClassError {
    /// Login credentials are incorrect.
    #[error("authentication failed")]
    AuthenticationFailed,
    /// The user has not logged in.
    #[error("user not logged in")]
    NotLoggedIn,
    /// Cyper-related error.
    #[error("cyper error: {0}")]
    CyperError(CyperError),
    /// Error parsing data from the server.
    #[error("data parsing error")]
    DataParsingError,
}

/// Generic response structure from the iClass API.
#[derive(Clone, Debug, Deserialize)]
pub struct Response<T> {
    /// The status code of the response, 0 for success.
    #[serde(rename = "STATUS")]
    pub status: String,
    /// Optional error code.
    ///
    /// - 100: 参数错误
    /// - 106: 用户不存在
    /// - 107: 密码错误
    #[serde(rename = "ERRCODE")]
    pub err_code: Option<String>,
    /// Optional error message.
    #[serde(rename = "ERRMSG")]
    pub err_msg: Option<String>,
    /// The result data.
    pub result: Option<T>,
}

impl IClass {
    /// Creates a new instance of [`IClass`].
    pub fn new() -> Self {
        Self::with_client(Client::new())
    }

    /// Creates a new instance of [`IClass`] with a custom [`Client`].
    pub fn with_client(client: Client) -> Self {
        Self { client, login_result: None }
    }
}

impl From<CyperError> for IClassError {
    fn from(e: CyperError) -> Self {
        IClassError::CyperError(e)
    }
}

impl<T> Response<T> {
    /// Converts the response into a [`Result`].
    pub fn into_result(self) -> Result<T, IClassError> {
        match self.status.as_str() {
            "0" => {
                if let Some(result) = self.result {
                    Ok(result)
                } else {
                    Err(IClassError::DataParsingError)
                }
            }
            "106" | "107" => Err(IClassError::AuthenticationFailed),
            _ => Err(IClassError::DataParsingError),
        }
    }
}
