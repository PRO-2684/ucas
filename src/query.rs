//! Query selected courses.

use std::fmt::Display;

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
    #[serde(
        rename = "yearStatus",
        deserialize_with = "super::util::deserialize_str_to_bool"
    )]
    pub is_current: bool,
}

/// A course.
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Course {
    // /// Course ID in iClass system.
    // pub id: String,
    /// Course ID as we all know.
    #[serde(rename = "courseNum")]
    pub course_id: String,
    /// Course name. There may be courses with the same name.
    pub course_name: String,
    /// Classroom name.
    pub classroom_name: String,
    /// Teacher name.
    pub teacher_name: String,
}

/// A daily schedule.
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DailySchedule {
    /// Date of this schedule.
    pub date_str: String,
    /// Schedules in this day.
    #[serde(rename = "schedData")]
    pub schedules: Vec<Schedule>,
}

/// A single schedule entry.
#[derive(Clone, Debug, Deserialize)]
pub struct Schedule {
    /// The course scheduled.
    #[serde(flatten)]
    pub course: Course,
    /// Id of this schedule.
    pub id: String,
    /// Unique id of this schedule.
    pub uuid: String,
    /// Begin time.
    #[serde(rename = "classBeginTime")]
    pub begin_time: String,
    /// End time.
    #[serde(rename = "classEndTime")]
    pub end_time: String,
}

impl IClass {
    /// Queries current semester.
    ///
    /// # Errors
    ///
    /// See [`IClassError`].
    #[allow(deprecated, reason = "This is what the API returns")]
    #[deprecated(note = "Not used in the mini-app")]
    pub async fn query_semester(&self) -> Result<Vec<Semester>, IClassError> {
        let url = self
            .api_root
            .join("app/course/get_base_school_year.action")?;
        let response: Response<Vec<Semester>> = self.client.get(url)?.send().await?.json().await?;
        let semesters = response.into_result()?;
        Ok(semesters)
    }

    /// Queries selected courses for current semester.
    ///
    /// # Errors
    ///
    /// See [`IClassError`].
    pub async fn query_courses(&self) -> Result<Vec<Course>, IClassError> {
        let user_session = self.get_user_session()?;
        let url = self.api_root.join("app/my/get_my_course.action")?;
        let response: Response<Vec<Course>> = self
            .client
            .get(url)?
            .header("sessionId", &user_session.session_id)?
            .query(&[("id", &user_session.id)])? // FIXME: Using form?
            .send()
            .await?
            .json()
            .await?;
        let courses = response.into_result()?;

        Ok(courses)
    }

    /// Queries daily schedule.
    ///
    /// # Arguments
    ///
    /// - `date` - A date string in "YYYYMMDD" format within the week to query, like "20251013".
    ///
    /// # Errors
    ///
    /// See [`IClassError`].
    pub async fn query_daily_schedule(&self, date: &str) -> Result<Vec<Schedule>, IClassError> {
        let user_session = self.get_user_session()?;
        let url = self
            .api_root
            .join("app/course/get_stu_course_sched.action")?;
        let response: Response<Vec<Schedule>> = self
            .client
            .get(url)?
            .header("sessionId", &user_session.session_id)?
            .query(&[("id", user_session.id.as_str()), ("dateStr", date)])?
            .send()
            .await?
            .json()
            .await?;
        let daily_schedule = response.into_result()?;

        Ok(daily_schedule)
    }

    /// Queries weekly schedule.
    ///
    /// # Arguments
    ///
    /// - `date` - A date string in "YYYYMMDD" format within the week to query, like "20251013".
    ///
    /// # Errors
    ///
    /// See [`IClassError`].
    pub async fn query_weekly_schedule(
        &self,
        date: &str,
    ) -> Result<Vec<DailySchedule>, IClassError> {
        let user_session = self.get_user_session()?;
        let url = self
            .api_root
            .join("app/course/get_stu_course_sched_week.action")?;
        let response: Response<Vec<DailySchedule>> = self
            .client
            .get(url)?
            .header("sessionId", &user_session.session_id)?
            .query(&[("id", user_session.id.as_str()), ("dateStr", date)])?
            .send()
            .await?
            .json()
            .await?;
        let week_schedule = response.into_result()?;

        Ok(week_schedule)
    }
}

impl Display for Course {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self {
            course_name,
            course_id,
            teacher_name,
            classroom_name,
            ..
        } = self;
        write!(
            f,
            "{course_name} ({course_id}) - {teacher_name} @ {classroom_name}"
        )
    }
}

impl Display for Schedule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self {
            course,
            uuid,
            begin_time,
            end_time,
            ..
        } = self;
        write!(
            f,
            "[{uuid} {begin_time} ~ {end_time}] {}",
            course.course_name
        )
    }
}

impl Display for DailySchedule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self {
            date_str,
            schedules,
        } = self;
        writeln!(f, "Schedule on {date_str}:")?;
        for schedule in schedules {
            writeln!(f, "  {schedule}")?;
        }
        Ok(())
    }
}
