mod controller;
mod flash;
mod middleware;
mod tools;

use axum::{
    http::StatusCode,
    middleware as axum_middleware,
    routing::{delete, get, get_service, post},
    Router,
};

use middleware::auth::Auth;
use migration::{Migrator, MigratorTrait};
use service::sea_orm::Database;

use std::env;
use tera::Tera;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;

use crate::controller::user::UserController;

use tools::AppState;

#[tokio::main]
async fn start() -> anyhow::Result<()> {
    env::set_var("RUST_LOG", "debug");
    tracing_subscriber::fmt::init();

    dotenvy::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let server_url = format!("{host}:{port}");

    let conn = Database::connect(db_url)
        .await
        .expect("Database connection failed");
    Migrator::up(&conn, None).await.unwrap();

    let templates = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*"))
        .expect("Tera initialization failed");

    let state = AppState { templates, conn };

    let app = Router::new()
        .route("/api/login", post(UserController::login))
        .route(
            "/user",
            get(UserController::list_users).layer(axum_middleware::from_fn_with_state(
                state.clone(),
                Auth::authorization_middleware,
            )),
        )
        .route(
            "/user/:id",
            get(UserController::get_user_by_id).layer(axum_middleware::from_fn_with_state(
                state.clone(),
                Auth::authorization_middleware,
            )),
        )
        .route(
            "/user/new",
            post(UserController::create_user).layer(axum_middleware::from_fn_with_state(
                state.clone(),
                Auth::authorization_middleware,
            )),
        )
        .route(
            "/user/update/:id",
            post(UserController::update_user).layer(axum_middleware::from_fn_with_state(
                state.clone(),
                Auth::authorization_middleware,
            )),
        )
        .route(
            "/user/delete/:id",
            delete(UserController::delete_user).layer(axum_middleware::from_fn_with_state(
                state.clone(),
                Auth::authorization_middleware,
            )),
        )
        .nest_service(
            "/static",
            get_service(ServeDir::new(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/static"
            )))
            .handle_error(|error| async move {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Unhandled internal error: {error}"),
                )
            }),
        )
        .layer(CookieManagerLayer::new())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(&server_url).await.unwrap();
    axum::serve(listener, app).await?;

    Ok(())
}

pub fn main() {
    let result = start();

    if let Some(err) = result.err() {
        println!("Error: {err}");
    }
}
