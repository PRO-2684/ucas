//! Login related logic.

use super::{IClass, IClassError, Response};
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{BufReader, BufWriter, Error as IoError, Write},
    path::Path,
};

impl IClass {
    /// Logs in to the iClass platform.
    ///
    /// # Errors
    ///
    /// See [`IClassError`].
    pub async fn login(&mut self, username: &str, password: &str) -> Result<(), IClassError> {
        // /app/user/login.action
        let url = self.api_root.join("app/user/login.action")?;
        let response: Response<UserSessionInfo> = self
            .client
            .post(url)?
            .query(&[("phone", username), ("password", password)])?
            .send()
            .await?
            .json()
            .await?;
        let login_result = response.into_result()?;
        self.user_session.replace(login_result);

        Ok(())
    }

    /// Restores user session from given file.
    ///
    /// # Errors
    ///
    /// IO errors during file operations.
    pub fn restore_session_from_file<P: AsRef<Path>>(&mut self, path: P) -> Result<(), IoError> {
        let session_info = UserSessionInfo::load_from_file(path)?;
        self.user_session.replace(session_info);
        Ok(())
    }

    /// Saves user session to given file, if any. Returns whether a session existed.
    ///
    /// # Errors
    ///
    /// IO errors during file operations.
    pub fn save_session_to_file<P: AsRef<Path>>(&self, path: P) -> Result<bool, IoError> {
        let exists = if let Some(session_info) = &self.user_session {
            session_info.save_to_file(path)?;
            true
        } else {
            false
        };
        Ok(exists)
    }
}

/// User session information returned after login.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserSessionInfo {
    /// ID of the user.
    pub id: String,
    /// Session ID.
    pub session_id: String,
    /// Real name.
    pub real_name: String,
    /// Student number.
    pub student_no: String,
}

impl UserSessionInfo {
    /// Saves the information to a file.
    ///
    /// # Errors
    ///
    /// [IO errors](IoError) during file operations.
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), IoError> {
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);
        serde_json::to_writer_pretty(&mut writer, self)?;
        writer.flush()?;
        Ok(())
    }

    /// Loads the information from a file.
    ///
    /// # Errors
    ///
    /// [IO errors](IoError) during file operations.
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, IoError> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let session_info = serde_json::from_reader(reader)?;
        Ok(session_info)
    }
}
