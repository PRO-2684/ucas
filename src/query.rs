//! Query selected courses.

use super::{API_ROOT, IClass, IClassError, Response};
use serde::Deserialize;

/// Query semester response structure.
pub type QuerySemesterResult = Vec<Semester>;

/// A semester.
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Semester {
    /// Semester code.
    pub code: String,
    /// Semester name.
    pub name: String,
    /// Semester begin date.
    pub begin_date: String,
    /// Semester end date.
    pub end_date: String,
    /// Year status - whether it is the current semester.
    #[serde(deserialize_with = "super::util::deserialize_str_to_bool")]
    pub year_status: bool,
}

/// Query courses response structure.
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryCoursesResult {}

impl IClass {
    /// Queries current semester.
    pub async fn query_semester(&self) -> Result<QuerySemesterResult, IClassError> {
        // /app/course/get_base_school_year.action
        let response: Response<QuerySemesterResult> = self
            .client
            .get(format!("{API_ROOT}app/course/get_base_school_year.action"))?
            .send()
            .await?
            .json()
            .await?;
        let semesters = response.into_result()?;
        Ok(semesters)
    }
    /// Queries selected courses for given `semester`.
    pub async fn query_courses(&self, semester: &str) -> Result<QueryCoursesResult, IClassError> {
        // /app/choosecourse/get_myall_course.action?xq_code={semester}
        todo!();
    }
}
