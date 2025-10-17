//! Login related logic.

use super::{IClass, API_ROOT, Response, IClassError};
use serde::Deserialize;

impl IClass {
    /// Logs in to the iClass platform.
    pub async fn login(&mut self, username: &str, password: &str) -> Result<(), IClassError> {
        // /app/user/login.action
        let response: Response<LoginResult> = self.client
            .post(format!("{API_ROOT}app/user/login.action"))?
            .query(&[
                ("phone", username),
                ("password", password),
            ])?
            .send()
            .await?
            .json()
            .await?;
        let login_result = response.into_result()?;
        self.login_result.replace(login_result);

        Ok(())
    }
}

/// Login response structure.
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginResult {
    /// ID of the user.
    pub id: String,
    /// Session ID.
    pub session_id: String,
    /// Real name.
    pub real_name: String,
    /// Student number.
    pub student_no: String,
}
