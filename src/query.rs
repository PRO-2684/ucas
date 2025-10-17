//! Query selected courses.

use super::{IClass, API_ROOT, Response, IClassError};
use serde::Deserialize;

/// Query response structure.
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryResult {}

impl IClass {
    /// Queries selected courses for given `term_id`.
    pub async fn query_courses(&self, term_id: &str) -> Result<QueryResult, IClassError> {
        // /app/choosecourse/get_myall_course.action?xq_code={term_id}
        todo!();
    }
}
