//! Check in to given class.

use super::{API_ROOT, IClass, IClassError, Response};
use serde::Deserialize;

/// Check-in response structure.
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckInResult {}

impl IClass {
    /// Checks in to a class with given `course_id`.
    pub async fn check_in(&self, course_id: &str) -> Result<CheckInResult, IClassError> {
        // /app/course/stu_scan_sign.action?courseSchedId={course_id}&timestamp={timestamp}
        todo!();
    }
}
