use crate::tools::{AppState, Params, ResponseData, ResponseStatus};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    Extension,
};
use entity::users::Model as UserModel;
use serde_json::json;
use service::{CartItemModel, CartItemServices};
use tera::to_value;

/// 购物车控制器
/// 处理所有与购物车相关的HTTP请求
pub struct CartController;

impl CartController {
    /// 获取用户的购物车列表
    /// 
    /// # 参数
    /// - user: 当前登录用户信息
    /// - state: 应用状态
    /// - params: 分页参数
    /// 
    /// # 返回
    /// - 购物车商品列表
    pub async fn list_cart_items(
        Extension(user): Extension<UserModel>,
        state: State<AppState>,
        Query(params): Query<Params>,
    ) -> Result<Json<serde_json::Value>, (StatusCode, &'static str)> {
        // 根据用户ID获取购物车商品列表
        let cart_items = CartItemServices::get_cart_items_by_user_id(&state.conn, user.id)
            .await
            .map_err(|e| {
                println!("Failed to get cart items: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to get cart items",
                )
            })?;

        // 构建响应数据
        let data = ResponseData {
            status: ResponseStatus::Success,
            data: {
                json!({
                    "cart_items": cart_items,
                })
            },
        };

        let json_data = to_value(data).unwrap();
        println!("Json data: {:?}", json_data);

        Ok(Json(json!(json_data)))
    }

    /// 创建新的购物车商品
    /// 
    /// # 参数
    /// - state: 应用状态
    /// - payload: 购物车商品数据
    /// 
    /// # 返回
    /// - 创建成功消息
    pub async fn create_cart_item(
        state: State<AppState>,
        Json(payload): Json<CartItemModel>,
    ) -> Result<Json<serde_json::Value>, (StatusCode, &'static str)> {
        // 创建购物车商品
        CartItemServices::create_cart_item(&state.conn, payload)
            .await
            .map_err(|e| {
                println!("Failed to create cart item: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to create cart item",
                )
            })?;

        Ok(Json(json!({
            "status": "success",
            "message": "Cart item created successfully"
        })))
    }

    /// 更新购物车商品信息
    /// 
    /// # 参数
    /// - state: 应用状态
    /// - id: 购物车商品ID
    /// - payload: 更新的购物车商品数据
    /// 
    /// # 返回
    /// - 更新成功消息
    pub async fn update_cart_item(
        state: State<AppState>,
        Path(id): Path<i32>,
        Json(payload): Json<CartItemModel>,
    ) -> Result<Json<serde_json::Value>, (StatusCode, &'static str)> {
        // 更新购物车商品
        CartItemServices::update_cart_item_by_id(&state.conn, id, payload)
            .await
            .map_err(|e| {
                println!("Failed to update cart item: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to update cart item",
                )
            })?;

        Ok(Json(json!({
            "status": "success",
            "message": "Cart item updated successfully"
        })))
    }

    /// 删除购物车商品
    /// 
    /// # 参数
    /// - state: 应用状态
    /// - id: 购物车商品ID
    /// 
    /// # 返回
    /// - 删除成功消息
    pub async fn delete_cart_item(
        state: State<AppState>,
        Path(id): Path<i32>,
    ) -> Result<Json<serde_json::Value>, (StatusCode, &'static str)> {
        // 删除购物车商品
        CartItemServices::delete_cart_item_by_id(&state.conn, id)
            .await
            .map_err(|e| {
                println!("Failed to delete cart item: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to delete cart item",
                )
            })?;

        Ok(Json(json!({
            "status": "success",
            "message": "Cart item deleted successfully"
        })))
    }
}
