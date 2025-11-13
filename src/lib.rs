//! # `ucas-iclass` library crate
//!
//! If you are reading this, you are reading the documentation for the `ucas-iclass` library crate. For the cli, kindly refer to the README file.

#![deny(missing_docs)]
#![warn(clippy::all, clippy::nursery, clippy::pedantic, clippy::cargo)]
#![allow(clippy::multiple_crate_versions, reason = "Dependency issues")]

mod checkin;
mod login;
mod query;
pub mod util;

pub use checkin::CheckInResult;
pub use login::UserSessionInfo;
pub use query::{Course, DailySchedule, Schedule, Semester};

use cyper::{Client, Error as CyperError};
use serde::Deserialize;
use std::fmt::Debug;
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
    /// The user has not logged in.
    #[error("user not logged in")]
    NotLoggedIn,
    /// Other API errors.
    #[error("API error: {0}")]
    ApiError(String),
    /// Cyper-related error.
    #[error("cyper error: {0}")]
    CyperError(#[from] CyperError),
    /// Error parsing data from the server.
    #[error("data parsing error")]
    DataParsingError,
}

/// Generic response structure from the iClass API.
#[derive(Clone, Debug, Deserialize)]
pub struct Response<T>
where
    T: Debug,
{
    /// The status code of the response, 0 for success.
    #[serde(rename = "STATUS", deserialize_with = "util::deserialize_str_to_int")]
    pub status: i8,
    /// Optional error code.
    ///
    /// - 100: 参数错误
    /// - 106: 用户不存在
    /// - 107: 密码错误
    #[serde(
        rename = "ERRCODE",
        default,
        deserialize_with = "util::deserialize_opt_str_to_int"
    )]
    pub err_code: Option<i8>,
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

impl From<ParseError> for IClassError {
    fn from(e: ParseError) -> Self {
        Self::CyperError(CyperError::UrlParse(e))
    }
}

impl<T> Response<T>
where
    T: Debug,
{
    /// Converts the response into a [`Result`], translating status codes into errors.
    ///
    /// # Errors
    ///
    /// See [`IClassError`].
    pub fn into_result(self) -> Result<T, IClassError> {
        if self.status == 0 {
            self.result.ok_or(IClassError::DataParsingError)
        } else {
            Err(IClassError::ApiError(if let Some(msg) = self.err_msg {
                msg
            } else {
                format!("Unknown error, {self:?}")
            }))
        }
    }
}
