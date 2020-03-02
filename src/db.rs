use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

pub type DbConn = PgConnection;
pub type DbPool = r2d2::Pool<ConnectionManager<DbConn>>;

pub type DBResult<T> = Result<T, diesel::result::Error>;