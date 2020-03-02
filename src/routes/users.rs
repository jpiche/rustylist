use actix_web::{delete, get, post, web, Error, HttpResponse};

use crate::db::*;
use crate::models::{NewUser, User, Paginated, PageQuery};

#[get("/users")]
pub async fn all(
    pool: web::Data<DbPool>,
    web::Query(query): web::Query<PageQuery>
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    let page = Paginated::from_query(&query);
    let users = User::list(page, &conn).map_err(|e| {
        eprintln!("db error: {:?}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    Ok(HttpResponse::Ok().json(users))
}

#[post("/users")]
pub async fn create(
    pool: web::Data<DbPool>,
    user: web::Json<NewUser>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    println!("new user json: {:?}", user.0);

    let user = User::insert(&user.0, &conn).map_err(|e| {
        eprintln!("db error: {:?}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    Ok(HttpResponse::Created().json(user))
}

#[get("/users/{id}")]
pub async fn get(
    pool: web::Data<DbPool>,
    path: web::Path<(i64,)>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    println!("get user by id: {}", path.0);
    let user = User::find_by_id(path.0, &conn).map_err(|e| {
        eprintln!("db error: {:?}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    match user {
        Some(user) => {
            println!("found user: {:#?}", user);
            Ok(HttpResponse::Ok().json(user))
        }
        None => Ok(HttpResponse::NotFound().finish()),
    }
}

#[delete("/users/{id}")]
pub async fn delete(
    pool: web::Data<DbPool>,
    path: web::Path<(i64,)>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    let user_id = path.0;
    println!("delete user by id: {}", user_id);
    Ok(HttpResponse::NotImplemented().finish())
}
