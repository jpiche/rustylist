use actix_web::{get, post, delete, web, Error, HttpResponse};

use crate::db::*;
use crate::models::{NewTask, Task, TaskUpdate, Paginated, PageQuery};

#[get("/tasks")]
pub async fn all(
    pool: web::Data<DbPool>,
    web::Query(query): web::Query<PageQuery>
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    let page = Paginated::from_query(&query);
    let tasks = Task::list(page, &conn).map_err(|e| {
        eprintln!("db error: {:?}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    Ok(HttpResponse::Ok().json(tasks))
}

#[post("/tasks")]
pub async fn create(
    pool: web::Data<DbPool>,
    task: web::Json<NewTask>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    println!("new task json: {:?}", task.0);

    let task = Task::insert(&task.0, &conn).map_err(|e| {
        eprintln!("db error: {:?}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    Ok(HttpResponse::Created().json(task))
}

#[get("/tasks/{id}")]
pub async fn get(
    pool: web::Data<DbPool>,
    path: web::Path<(i64,)>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");
    let task_id = path.0;

    println!("get task by id: {}", task_id);
    let task = Task::find_by_id(task_id, &conn).map_err(|e| {
        eprintln!("db error: {:?}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    match task {
        Some(task) => {
            println!("found task: {:#?}", task);
            Ok(HttpResponse::Ok().json(task))
        }
        None => Ok(HttpResponse::NotFound().finish()),
    }
}

#[post("/tasks/{id}")]
pub async fn update(
    pool: web::Data<DbPool>,
    task: web::Json<TaskUpdate>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    let task = task.0;
    println!("update task with json: {:?}", task);

    let new_task = Task::update(task, &conn).map_err(|e| {
        eprintln!("db error: {:?}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    match new_task {
        Some(task) => {
            println!("updated task: {:#?}", task);
            Ok(HttpResponse::Ok().json(task))
        }
        None => Ok(HttpResponse::NotFound().finish()),
    }
}

#[get("/users/{id}/tasks")]
pub async fn get_for_user(
    pool: web::Data<DbPool>,
    path: web::Path<(i64,)>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    let user_id = path.0;
    println!("get tasks for user_id: {}", user_id);

    let tasks = Task::find_by_user_id(user_id, &conn).map_err(|e| {
        eprintln!("db error: {:?}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    Ok(HttpResponse::Ok().json(tasks))
}

#[delete("/tasks/{id}")]
pub async fn delete(
    pool: web::Data<DbPool>,
    path: web::Path<(i64,)>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    let task_id = path.0;
    println!("delete task by id: {}", task_id);

    let tasks_deleted = Task::delete(task_id, &conn).map_err(|e| {
        eprintln!("db error: {:?}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    match tasks_deleted {
        0 => Ok(HttpResponse::NotFound().finish()),
        _ => Ok(HttpResponse::NoContent().finish()),
    }
}