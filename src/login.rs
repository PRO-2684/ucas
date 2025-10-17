//! Login related logic.

use super::{IClass, IClassError, Response};
use serde::Deserialize;

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
}

/// User session information returned after login.
#[derive(Clone, Debug, Deserialize)]
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
