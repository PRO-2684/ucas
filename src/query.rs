//! Query selected courses.

use super::{IClass, IClassError, Response};
use serde::Deserialize;

/// A semester.
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[deprecated(note = "Not used in the mini-app")]
pub struct Semester {
    /// Semester code.
    pub code: String,
    /// Semester name.
    pub name: String,
    /// Semester begin date.
    pub begin_date: String,
    /// Semester end date.
    pub end_date: String,
    /// Whether it is the current semester.
    #[serde(rename = "yearStatus", deserialize_with = "super::util::deserialize_str_to_bool")]
    pub is_current: bool,
}

/// A course.
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Course {
    /// Course ID in iClass system.
    pub id: String,
    /// Course ID as we all know.
    #[serde(rename = "courseNum")]
    pub course_id: String,
    /// Course name. There may be courses with the same name.
    pub name: String,
    /// Course begin date.
    pub begin_date: String,
    /// Course end date.
    pub end_date: String,
    /// Classroom name.
    pub classroom_name: String,
    /// Teacher name.
    pub teacher_name: String,
}

impl IClass {
    /// Queries current semester.
    #[allow(deprecated, reason = "This is what the API returns")]
    #[deprecated(note = "Not used in the mini-app")]
    pub async fn query_semester(&self) -> Result<Vec<Semester>, IClassError> {
        // /app/course/get_base_school_year.action
        let url = self
            .api_root
            .join("app/course/get_base_school_year.action")?;
        let response: Response<Vec<Semester>> = self.client.get(url)?.send().await?.json().await?;
        let semesters = response.into_result()?;
        Ok(semesters)
    }

    /// Queries selected courses for current semester.
    pub async fn query_courses(&self) -> Result<Vec<Course>, IClassError> {
        let Some(login_result) = &self.login_result else {
            return Err(IClassError::NotLoggedIn);
        };
        let url = self
            .api_root
            .join("app/my/get_my_course.action")?;
        let response: Response<Vec<Course>> = self
            .client
            .get(url)?
            .header("sessionId", &login_result.session_id)?
            .query(&[("id", &login_result.id)])? // FIXME: Using form?
            .send()
            .await?
            .json()
            .await?;
        let courses = response.into_result()?;

        Ok(courses)
    }
}
