//! Query selected courses.

use chrono::{DateTime, FixedOffset, NaiveDate};
use std::fmt;

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
    #[serde(rename = "dateStr", deserialize_with = "super::util::deserialize_str_to_date")]
    pub date: NaiveDate,
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
    /// Check in status. Only work in [`query_daily_schedule`](IClass::query_daily_schedule), and does not work in [`query_weekly_schedule`](IClass::query_weekly_schedule) or when wrapped in [`DailySchedule`](DailySchedule).
    #[serde(rename = "signStatus", deserialize_with = "super::util::deserialize_str_to_bool")]
    pub checked_in: bool,
    /// Begin time.
    #[serde(rename = "classBeginTime", deserialize_with = "super::util::deserialize_str_to_datetime")]
    pub begin_time: DateTime<FixedOffset>,
    /// End time.
    #[serde(rename = "classEndTime", deserialize_with = "super::util::deserialize_str_to_datetime")]
    pub end_time: DateTime<FixedOffset>,
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

    // https://iclass.ucas.edu.cn:8181/app/choosecourse/get_myall_course.action?user_type=1

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
    pub async fn query_daily_schedule(&self, date: &NaiveDate) -> Result<Vec<Schedule>, IClassError> {
        let user_session = self.get_user_session()?;
        let url = self
            .api_root
            .join("app/course/get_stu_course_sched.action")?;
        let date_str = super::util::format_date_to_str(date);
        let response: Response<Vec<Schedule>> = self
            .client
            .get(url)?
            .header("sessionId", &user_session.session_id)?
            .query(&[("id", &user_session.id), ("dateStr", &date_str)])?
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
        date: &NaiveDate,
    ) -> Result<Vec<DailySchedule>, IClassError> {
        let user_session = self.get_user_session()?;
        let url = self
            .api_root
            .join("app/course/get_stu_course_sched_week.action")?;
        let date_str = super::util::format_date_to_str(date);
        let response: Response<Vec<DailySchedule>> = self
            .client
            .get(url)?
            .header("sessionId", &user_session.session_id)?
            .query(&[("id", &user_session.id), ("dateStr", &date_str)])?
            .send()
            .await?
            .json()
            .await?;
        let week_schedule = response.into_result()?;

        Ok(week_schedule)
    }
}

impl fmt::Display for Course {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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

impl fmt::Display for Schedule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            course,
            id,
            uuid,
            checked_in,
            begin_time,
            end_time,
            ..
        } = self;
        let indicator = if *checked_in { "[✓]" } else { "[ ]" };
        let (begin_time, end_time) = (
            super::util::format_datetime_to_str(begin_time),
            super::util::format_datetime_to_str(end_time),
        );
        write!(
            f,
            "{indicator} [{begin_time} ~ {end_time}] id={id} uuid={uuid} {}",
            course.course_name
        )
    }
}

impl fmt::Display for DailySchedule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            date,
            schedules,
        } = self;
        writeln!(f, "Schedule on {date}:")?;
        for schedule in schedules {
            let Schedule {
                id,
                uuid,
                begin_time,
                end_time,
                ..
            } = schedule;
            let (begin_time, end_time) = (
                super::util::format_datetime_to_str(begin_time),
                super::util::format_datetime_to_str(end_time),
            );
            writeln!(f, "  [{begin_time} ~ {end_time}] id={id} uuid={uuid} {}", schedule.course.course_name)?;
        }
        Ok(())
    }
}
