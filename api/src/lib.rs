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

use controller::{
    banner::BannerController, category::CategoriesController, order::OrderController,
};
use middleware::auth::Auth;
use migration::{Migrator, MigratorTrait};
use service::sea_orm::Database;

use std::env;
use tera::Tera;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;

use crate::controller::cart::CartController;
use crate::controller::porduct::PorductController;
use crate::controller::reviews::ReviewController;
use crate::controller::user::UserController;

use tools::AppState;

/// 应用程序入口函数
/// 负责初始化数据库连接、模板引擎和路由配置
#[tokio::main]
async fn start() -> anyhow::Result<()> {
    // 设置日志级别
    env::set_var("RUST_LOG", "debug");
    tracing_subscriber::fmt::init();

    // 加载环境变量
    dotenvy::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let server_url = format!("{host}:{port}");

    // 连接数据库并执行迁移
    let conn = Database::connect(db_url)
        .await
        .expect("Database connection failed");
    Migrator::up(&conn, None).await.unwrap();

    // 初始化模板引擎
    let templates = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*"))
        .expect("Tera initialization failed");

    // 创建应用状态
    let state = AppState { templates, conn };

    // 配置路由
    let app = Router::new()
        // 用户认证相关路由
        .route("/api/login", post(UserController::login))
        // 用户管理相关路由
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
        // 商品管理相关路由
        .route("/product/get", get(PorductController::list_porducts))
        .route("/product/create", post(PorductController::create_porduct))
        .route(
            "/product/update/:id",
            post(PorductController::update_porduct),
        )
        .route(
            "/product/delete/:id",
            delete(PorductController::delete_porduct),
        )
        // 分类管理相关路由
        .route("/category/get", get(CategoriesController::list_categories))
        .route(
            "/category/create",
            post(CategoriesController::create_category),
        )
        .route(
            "/category/update/:id",
            post(CategoriesController::update_category),
        )
        .route(
            "/category/delete/:id",
            delete(CategoriesController::delete_category),
        )
        // 订单管理相关路由
        .route("/order/create", post(OrderController::create_order))
        .route(
            "/order/update_status/:id",
            post(OrderController::update_order_status),
        )
        .route(
            "/order/set_payment/:id",
            post(OrderController::set_payment_status),
        )
        .route(
            "/order/cancel_order/:id",
            post(OrderController::cancel_order),
        )
        // 轮播图管理相关路由
        .route("/banner/all", get(BannerController::list_banners_all))
        .route("/banner/create", post(BannerController::create_banner))
        .route("/banner/update/:id", post(BannerController::update_banner))
        .route(
            "/banner/delete/:id",
            delete(BannerController::delete_banner),
        )
        // 购物车管理相关路由
        .route("/cart/get", get(CartController::list_cart_items))
        .route("/cart/create", post(CartController::create_cart_item))
        .route("/cart/update/:id", post(CartController::update_cart_item))
        .route("/cart/delete/:id", delete(CartController::delete_cart_item))
        // 评论管理相关路由
        .route("/review/get", get(ReviewController::list_reviews))
        .route("/review/create", post(ReviewController::create_review))
        .route(
            "/review/update/:id",
            post(ReviewController::update_review_by_id),
        )
        .route(
            "/review/delete/:id",
            delete(ReviewController::delete_review_by_id),
        )
        .route(
            "/review/get_reviews_by_product_id/:id",
            get(ReviewController::get_reviews_by_product_id),
        )
        // 静态文件服务
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

    // 启动服务器
    let listener = tokio::net::TcpListener::bind(&server_url).await.unwrap();
    axum::serve(listener, app).await?;

    Ok(())
}

/// 程序入口点
pub fn main() {
    let result = start();

    if let Some(err) = result.err() {
        println!("Error: {err}");
    }
}
