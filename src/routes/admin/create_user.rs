use crate::authentication::compute_password_hash;
use crate::utils::see_other;
use actix_web::http::header::ContentType;
use actix_web::{web, HttpResponse};
use anyhow::Context;
use secrecy::{ExposeSecret, Secret};
use sqlx::PgPool;
use uuid::Uuid;

// yolo - creating new users from a form rather than seeding them in a migration
pub async fn create_user_form() -> Result<HttpResponse, actix_web::Error> {
    Ok(HttpResponse::Ok().content_type(ContentType::html()).body(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta http-equiv="content-type" content="text/html; charset=utf-8">
    <title>Create a new user</title>
</head>
<body>
    <form name="createUserForm" action="/create-user" method="post">
        <input type="text" name="username" placeholder="Username">
        <input type="password" name="password" placeholder="Password">
        <input type="submit" value="Create user">
    </form>
</body>
</html>"#,
    ))
}

pub async fn create_user_handler(
    form: web::Form<NewUser>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    match create_user(form.0.username, form.0.password, pool.get_ref()).await {
        Ok(_) => Ok(see_other("/login")),
        Err(_) => Ok(HttpResponse::InternalServerError().finish()),
    }
}

#[derive(serde::Deserialize)]
pub struct NewUser {
    username: String,
    password: Secret<String>,
}

#[tracing::instrument(name = "Create a new user", skip(password, pool))]
async fn create_user(
    username: String,
    password: Secret<String>,
    pool: &PgPool,
) -> Result<(), anyhow::Error> {
    let user_id = Uuid::new_v4();
    let password = match compute_password_hash(password) {
        Ok(password) => password,
        Err(e) => return Err(e),
    };
    sqlx::query!(
        r#"
        INSERT INTO users (user_id, username, password_hash)
        VALUES ($1, $2, $3)
        "#,
        user_id,
        username,
        password.expose_secret(),
    )
    .execute(pool)
    .await
    .context("Failed to perform a query to create a new user")?;

    Ok(())
}
