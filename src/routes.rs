use actix_web::{get, Error, HttpResponse};

pub mod tasks;
pub mod users;

#[get("/")]
pub async fn index() -> Result<HttpResponse, Error> {
    let body = r##"
Routes index
    GET     /
        This index

Users
    GET     /users
        List all users
    POST    /users
        Create new user; accepts JSON
    GET     /users/{id}
        Get a specific user by ID

Tasks
    GET     /tasks
        List all tasks
    POST    /tasks
        Create new task; accepts JSON
    GET     /task/{id}
        Get a specific task by ID
    DELETE  /task/{id}
        Delete a specific task by ID
    GET     /users/{id}/tasks
        List all tasks for a user
"##;

    Ok(HttpResponse::Ok().body(body))
}