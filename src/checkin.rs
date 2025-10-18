//! Check in to given class.

use super::{IClass, IClassError, Response};
use serde::Deserialize;
use std::{fmt, time::{SystemTime, UNIX_EPOCH}};

/// Check-in response structure.
#[derive(Clone, Debug, Deserialize)]
pub struct CheckInResult {
    // Object {"stuSignId": String("41254913"), "stuSignStatus": String("1")}
    /// Check-in ID. Maybe the serial number of the check-in record.
    #[serde(rename = "stuSignId")]
    pub id: String,
    /// Check-in status.
    #[serde(rename = "stuSignStatus", deserialize_with = "super::util::deserialize_str_to_bool")]
    pub status: bool,
}

impl IClass {
    /// Checks in the schedule with given uuid. This is equivalent to scanning the QR code on the smart device outside the classroom.
    ///
    /// # Errors
    ///
    /// See [`IClassError`].
    ///
    /// # Panics
    ///
    /// This function will panic if system time is before [`UNIX_EPOCH`].
    pub async fn check_in_by_uuid(&self, schedule_uuid: &str) -> Result<CheckInResult, IClassError> {
        // /app/course/stu_scan_sign.action?timeTableId={schedule_uuid}&timestamp={timestamp}
        let user_session = self.get_user_session()?;
        let url = self.api_root.join("app/course/stu_scan_sign.action")?;
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis();
        let response: Response<CheckInResult> = self
            .client
            .get(url)?
            .header("sessionId", &user_session.session_id)?
            .query(&[
                ("timeTableId", schedule_uuid),
                ("timestamp", &timestamp.to_string()),
                ("id", user_session.id.as_str()),
            ])?
            .send()
            .await?
            .json()
            .await?;
        let check_in_result = response.into_result()?;

        Ok(check_in_result)
    }

    /// Checks in the schedule with given id. This is equivalent to scanning the QR code on the computer inside the classroom.
    ///
    /// # Errors
    ///
    /// See [`IClassError`].
    ///
    /// # Panics
    ///
    /// This function will panic if system time is before [`UNIX_EPOCH`].
    pub async fn check_in_by_id(&self, schedule_id: &str) -> Result<CheckInResult, IClassError> {
        // /app/course/stu_sign_in.action?scheduleId={schedule_id}&timestamp={timestamp}
        let user_session = self.get_user_session()?;
        let url = self.api_root.join("app/course/stu_scan_sign.action")?;
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis();
        let response: Response<CheckInResult> = self
            .client
            .get(url)?
            .header("sessionId", &user_session.session_id)?
            .query(&[
                ("courseSchedId", schedule_id),
                ("timestamp", &timestamp.to_string()),
                ("id", user_session.id.as_str()),
            ])?
            .send()
            .await?
            .json()
            .await?;
        let check_in_result = response.into_result()?;

        Ok(check_in_result)
    }
}

impl fmt::Display for CheckInResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self { id, status  } = self;
        let status = if *status { "Success" } else { "Failed" };
        write!(f, "{status} (#{id})")
    }
}
