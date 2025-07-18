mod controller;
mod flash;
mod middleware;
mod tools;

use axum::{
    http::{Method, StatusCode},
    middleware as axum_middleware,
    routing::{delete, get, get_service, post},
    Router,
};

use controller::{
    banner::BannerController, category::CategoryController, order::OrderController,
    payment::PaymentController,
};
use middleware::auth::Auth;
use migration::{Migrator, MigratorTrait};
use service::sea_orm::Database;

use std::env;
use tera::Tera;
use tower_cookies::CookieManagerLayer;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;

use crate::controller::cart::CartController;
use crate::controller::porduct::PorductController;
use crate::controller::reviews::ReviewController;
use crate::controller::user::UserController;
use crate::controller::address::AddressController;
use crate::controller::invoice::InvoiceController;

use tools::AppState;

/// 应用程序入口函数
/// 负责初始化数据库连接、模板引擎和路由配置
#[tokio::main]
async fn start() -> anyhow::Result<()> {
    // 设置日志级别
    env::set_var("RUST_LOG", "debug");
    tracing_subscriber::fmt::init();
    let cors = CorsLayer::new()
        .allow_origin(Any) // 允许所有来源，生产环境建议指定具体来源
        .allow_methods([Method::GET, Method::POST, Method::DELETE]) // 允许的 HTTP 方法
        .allow_headers(Any); // 允许所有请求头

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
        .route("/api/register", post(UserController::register))
        // 用户管理相关路由
        .route(
            "/api/user",
            get(UserController::list_users).layer(axum_middleware::from_fn_with_state(
                state.clone(),
                Auth::authorization_middleware,
            )),
        )
        .route(
            "/api/user/:id",
            get(UserController::get_user_by_id).layer(axum_middleware::from_fn_with_state(
                state.clone(),
                Auth::authorization_middleware,
            )),
        )
        .route(
            "/api/user/new",
            post(UserController::create_user).layer(axum_middleware::from_fn_with_state(
                state.clone(),
                Auth::authorization_middleware,
            )),
        )
        .route(
            "/api/user/update/:id",
            post(UserController::update_user).layer(axum_middleware::from_fn_with_state(
                state.clone(),
                Auth::authorization_middleware,
            )),
        )
        .route(
            "/api/user/delete/:id",
            delete(UserController::delete_user).layer(axum_middleware::from_fn_with_state(
                state.clone(),
                Auth::authorization_middleware,
            )),
        )
        // 商品管理相关路由
        .route("/api/product/get", get(PorductController::list_porducts))
        .route(
            "/api/product/get/{id}",
            get(PorductController::get_product_by_id),
        )
        .route(
            "/api/product/create",
            post(PorductController::create_porduct).layer(axum_middleware::from_fn_with_state(
                state.clone(),
                Auth::authorization_middleware,
            )),
        )
        .route(
            "/api/product/update/:id",
            post(PorductController::update_porduct).layer(axum_middleware::from_fn_with_state(
                state.clone(),
                Auth::authorization_middleware,
            )),
        )
        .route(
            "/api/product/delete/:id",
            delete(PorductController::delete_porduct).layer(axum_middleware::from_fn_with_state(
                state.clone(),
                Auth::authorization_middleware,
            )),
        )
        .route(
            "/api/product/home",
            get(PorductController::get_product_by_home_product_type),
        )
        .route(
            "/api/product/add_home_product",
            post(PorductController::create_home_product).layer(
                axum_middleware::from_fn_with_state(state.clone(), Auth::authorization_middleware),
            ),
        )
        .route(
            "/api/product/delete_home_product/:id",
            delete(PorductController::delete_home_product).layer(
                axum_middleware::from_fn_with_state(state.clone(), Auth::authorization_middleware),
            ),
        )
        .route(
            "/api/product/new",
            get(PorductController::get_product_by_new),
        )
        .route(
            "/api/product/get/:id",
            get(PorductController::get_product_by_id),
        )
        // 分类管理相关路由
        .route(
            "/api/category/get",
            get(CategoryController::list_categories),
        )
        .route(
            "/api/category/create",
            post(CategoryController::create_category).layer(axum_middleware::from_fn_with_state(
                state.clone(),
                Auth::authorization_middleware,
            )),
        )
        .route(
            "/api/category/update/:id",
            post(CategoryController::update_category).layer(axum_middleware::from_fn_with_state(
                state.clone(),
                Auth::authorization_middleware,
            )),
        )
        .route(
            "/api/category/delete/:id",
            delete(CategoryController::delete_category).layer(
                axum_middleware::from_fn_with_state(state.clone(), Auth::authorization_middleware),
            ),
        )
        // 订单管理相关路由
        .route("/api/order/list", get(OrderController::list_orders).layer(axum_middleware::from_fn_with_state(
            state.clone(),
            Auth::authorization_middleware,
        )))
        .route("/api/order/create", post(OrderController::create_order).layer(
                axum_middleware::from_fn_with_state(state.clone(), Auth::authorization_middleware),
            ))
        .route(
            "/api/order/update_status/:id",
            post(OrderController::update_order_status).layer(axum_middleware::from_fn_with_state(
                state.clone(),
                Auth::authorization_middleware,
            )),
        )
        .route(
            "/api/order/set_payment/:id",
            post(OrderController::set_payment_status).layer(axum_middleware::from_fn_with_state(
                state.clone(),
                Auth::authorization_middleware,
            )),
        )
        .route(
            "/api/order/cancel_order/:id",
            post(OrderController::cancel_order)
                .layer(axum_middleware::from_fn_with_state(state.clone(), Auth::authorization_middleware)),
        )
        // 轮播图管理相关路由
        .route("/api/banner/all", get(BannerController::list_banners_all))
        .route(
            "/api/banner/create",
            post(BannerController::create_banner).layer(axum_middleware::from_fn_with_state(
                state.clone(),
                Auth::authorization_middleware,
            )),
        )
        .route(
            "/api/banner/update/:id",
            post(BannerController::update_banner).layer(axum_middleware::from_fn_with_state(
                state.clone(),
                Auth::authorization_middleware,
            )),
        )
        .route(
            "/api/banner/delete/:id",
            delete(BannerController::delete_banner).layer(axum_middleware::from_fn_with_state(
                state.clone(),
                Auth::authorization_middleware,
            )),
        )
        // 购物车管理相关路由
        .route("/api/cart/get", get(CartController::list_cart_items).layer(axum_middleware::from_fn_with_state(
            state.clone(),
            Auth::authorization_middleware,
        )))
        .route("/api/cart/create", post(CartController::create_cart_item))
        .route(
            "/api/cart/update/:id",
            post(CartController::update_cart_item),
        )
        .route(
            "/api/cart/delete/:id",
            delete(CartController::delete_cart_item),
        )
        // 评论管理相关路由
        .route("/api/review/get", get(ReviewController::list_reviews))
        .route("/api/review/create", post(ReviewController::create_review))
        .route(
            "/api/review/update/:id",
            post(ReviewController::update_review_by_id),
        )
        .route(
            "/api/review/delete/:id",
            delete(ReviewController::delete_review_by_id),
        )
        .route(
            "/api/review/get_reviews_by_product_id/:id",
            get(ReviewController::get_reviews_by_product_id),
        )
        .route("/api/payment", post(PaymentController::create_payment))
        // 地址管理相关路由
        .route("/api/address/create", post(AddressController::create_address).layer(axum_middleware::from_fn_with_state(
            state.clone(),
            Auth::authorization_middleware,
        )))
        .route("/api/address/update/:id", post(AddressController::update_address).layer(axum_middleware::from_fn_with_state(
            state.clone(),
            Auth::authorization_middleware,
        )))
        .route("/api/address/delete/:id", delete(AddressController::delete_address).layer(axum_middleware::from_fn_with_state(
            state.clone(),
            Auth::authorization_middleware,
        )))
        .route("/api/address/get", get(AddressController::get_addresses_by_user_id).layer(axum_middleware::from_fn_with_state(
            state.clone(),
            Auth::authorization_middleware,
        )))
        .route("/api/address/get/:id", get(AddressController::get_address_by_id))
        // 发票管理相关路由
        .route("/api/invoice/create", post(InvoiceController::create_invoice).layer(axum_middleware::from_fn_with_state(
            state.clone(),
            Auth::authorization_middleware,
        )))
        .route("/api/invoice/update/:id", post(InvoiceController::update_invoice).layer(axum_middleware::from_fn_with_state(
            state.clone(),
            Auth::authorization_middleware,
        )))
        .route("/api/invoice/delete/:id", delete(InvoiceController::delete_invoice).layer(axum_middleware::from_fn_with_state(
            state.clone(),
            Auth::authorization_middleware,
        )))
        .route("/api/invoice/list", get(InvoiceController::get_invoices).layer(axum_middleware::from_fn_with_state(
            state.clone(),
            Auth::authorization_middleware,
        )))
        .route("/api/invoice/get/:id", get(InvoiceController::get_invoice).layer(axum_middleware::from_fn_with_state(
            state.clone(),
            Auth::authorization_middleware,
        )))
        .route("/api/invoice/default", get(InvoiceController::get_default_invoice).layer(axum_middleware::from_fn_with_state(
            state.clone(),
            Auth::authorization_middleware,
        )))
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
        .nest_service(
            "/uploads",
            get_service(ServeDir::new("./uploads")).handle_error(|error| async move {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Unhandled internal error: {error}"),
                )
            }),
        )
        .layer(cors) // 添加 CORS 中间件
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
