use chrono::{DateTime, Utc};

use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::tasks;
use crate::db::*;
use crate::models::Paginated;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable)]
pub struct Task {
    pub id: i64,
    pub user_id: i64,
    pub title: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[table_name="tasks"]
pub struct NewTask {
    pub user_id: i64,
    pub title: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Identifiable, AsChangeset)]
#[table_name="tasks"]
pub struct TaskUpdate {
    pub id: i64,
    pub user_id: Option<i64>,
    pub title: Option<String>,
    pub description: Option<String>,
}

impl Task {
    pub fn find_by_id(input_id: i64, conn: &DbConn) -> DBResult<Option<Task>> {
        use crate::schema::tasks::dsl::*;

        let result_task = tasks
            .find(input_id)
            .first::<Task>(conn)
            .optional()?;

        Ok(result_task)
    }

    pub fn find_by_user_id(input_user_id: i64, conn: &DbConn) -> DBResult<Vec<Task>> {
        use crate::schema::tasks::dsl::*;

        let results = tasks
            .filter(user_id.eq(input_user_id))
            .order(id.asc())
            .load::<Task>(conn)?;
        Ok(results)
    }

    pub fn insert(new_task: &NewTask, conn: &DbConn) -> DBResult<Task> {
        let result = diesel::insert_into(tasks::table)
            .values(new_task.clone())
            .get_result::<Task>(conn)?;
        Ok(result)
    }

    pub fn update(new_task: TaskUpdate, conn: &DbConn) -> DBResult<Option<Task>> {
        use crate::schema::tasks::dsl::*;

        let result = diesel::update(tasks.find(new_task.id))
            .set(new_task)
            .get_result::<Task>(conn)
            .optional()?;
        Ok(result)
    }

    pub fn list(page: Paginated, conn: &DbConn) -> DBResult<Vec<Task>> {
        use crate::schema::tasks::dsl::*;

        let results = tasks
            .order(id.asc())
            .offset(page.offset())
            .limit(page.per_page)
            .load::<Task>(conn)?;
        Ok(results)
    }

    pub fn delete(task_id: i64, conn: &DbConn) -> DBResult<usize> {
        use crate::schema::tasks::dsl::*;

        let rows_deleted = diesel::delete(tasks
            .filter(id.eq(task_id))
        ).execute(conn)?;
        // since we're querying on PK, this should always be 1 if successful
        Ok(rows_deleted)
    }
}