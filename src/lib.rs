//! # `ucas-iclass` library crate
//!
//! If you are reading this, you are reading the documentation for the `ucas-iclass` library crate. For the cli, kindly refer to the README file.

#![deny(missing_docs)]
#![warn(clippy::all, clippy::nursery, clippy::pedantic, clippy::cargo)]
#![allow(clippy::multiple_crate_versions, reason = "Dependency issues")]

mod checkin;
mod login;
mod query;
mod util;

pub use checkin::CheckInResult;
use cyper::{Client, Error as CyperError};
pub use login::UserSessionInfo;
#[allow(deprecated, reason = "Re-exporting for potential use")]
pub use query::{Course, Semester};
use serde::Deserialize;
use url::{ParseError, Url};

/// The root URL of the iClass platform.
pub const API_ROOT: &str = "https://iclass.ucas.edu.cn:8181/";

/// The iClass struct.
pub struct IClass {
    /// API root URL.
    api_root: Url,
    /// The HTTP client.
    client: Client,
    /// User session information.
    pub user_session: Option<UserSessionInfo>,
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
    /// Other API errors.
    #[error("API error: {0}")]
    ApiError(String),
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

impl Default for IClass {
    fn default() -> Self {
        Self::new()
    }
}

impl IClass {
    /// Creates a new instance of [`IClass`].
    #[allow(clippy::missing_panics_doc, reason = "URL is constant and valid")]
    #[must_use]
    pub fn new() -> Self {
        Self::with_api_root(Url::parse(API_ROOT).unwrap())
    }

    /// Creates a new instance of [`IClass`] with given API root URL.
    #[must_use]
    pub fn with_api_root(url: Url) -> Self {
        Self {
            api_root: url,
            client: Client::new(),
            user_session: None,
        }
    }

    /// Gets a reference to user session info, or raises [`IClassError::NotLoggedIn`].
    ///
    /// # Errors
    ///
    /// [`IClassError::NotLoggedIn`] if the user is not logged in.
    fn get_user_session(&self) -> Result<&UserSessionInfo, IClassError> {
        self.user_session.as_ref().ok_or(IClassError::NotLoggedIn)
    }
}

impl From<CyperError> for IClassError {
    fn from(e: CyperError) -> Self {
        Self::CyperError(e)
    }
}

impl From<ParseError> for IClassError {
    fn from(e: ParseError) -> Self {
        Self::CyperError(CyperError::UrlParse(e))
    }
}

impl<T> Response<T> {
    /// Converts the response into a [`Result`], translating status codes into errors.
    ///
    /// # Errors
    ///
    /// See [`IClassError`].
    pub fn into_result(self) -> Result<T, IClassError> {
        match self.status.as_str() {
            "0" => self
                .result
                .map_or_else(|| Err(IClassError::DataParsingError), |result| Ok(result)),
            "106" | "107" => Err(IClassError::AuthenticationFailed),
            _ => Err(IClassError::ApiError(
                self.err_msg.unwrap_or_else(|| "Unknown error".to_string()),
            )),
        }
    }
}
