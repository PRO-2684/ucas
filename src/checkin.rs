//! Check in to given class.

use super::{IClass, IClassError};
use serde::Deserialize;

/// Check-in response structure.
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckInResult {}

impl IClass {
    /// Checks in to a class with given `schedule_uuid`.
    ///
    /// # Errors
    ///
    /// See [`IClassError`].
    pub async fn check_in(&self, schedule_uuid: &str) -> Result<CheckInResult, IClassError> {
        // /app/course/stu_scan_sign.action?timeTableId={schedule_uuid}&timestamp={timestamp}
        todo!();
    }
}
