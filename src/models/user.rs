use chrono::{DateTime, Utc};

use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::users;
use crate::db::*;
use crate::models::Paginated;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[table_name="users"]
pub struct NewUser {
    pub name: String,
}

impl User {
    pub fn find_by_id(input_id: i64, conn: &DbConn) -> DBResult<Option<User>> {
        use crate::schema::users::dsl::*;

        let user = users
            .find(input_id)
            .first::<User>(conn)
            .optional()?;

        Ok(user)
    }

    pub fn insert(new_user: &NewUser, conn: &DbConn) -> DBResult<User> {
        let u = diesel::insert_into(users::table)
            .values(new_user.clone())
            .get_result::<User>(conn)?;
        Ok(u)
    }

    pub fn list(page: Paginated, conn: &DbConn) -> DBResult<Vec<User>> {
        use crate::schema::users::dsl::*;

        let results = users
            .order(id.asc())
            .offset(page.offset())
            .limit(page.per_page)
            .load::<User>(conn)?;
        Ok(results)
    }
}
