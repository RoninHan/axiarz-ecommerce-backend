use crate::tools::{AppState, Params, ResponseData, ResponseStatus};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    Extension,
};
use entity::{users::Model as UserModel};
use serde_json::json;
use serde_json::to_value;
use service::{
    sea_orm::prelude::Decimal,
    order_items::{OrderItemModel, OrderItemServices},
    orders::{OrderModel, OrderServices},
};

#[derive(serde::Deserialize, Debug)]
pub struct RequestData {
    total_price: Decimal,           // 订单总价
    coupon_code: Option<String>,    // 优惠券代码
    gift_card_code: Option<String>, // 礼品卡代码
    shipping_address: String,       // 收货地址
    billing_address: String,        // 发票地址
    payment_method: i32,            // 支付方式
    discount: Option<Decimal>,      // 优惠金额
    shipping_company: String,       // 快递公司
    tracking_number: String,        // 快递单号
    notes: Option<String>,          // 备注
    product_id: Option<i32>,        // 产品ID
    quantity: Option<i32>,          // 产品数量
    price: Option<Decimal>,         // 产品价格
}

#[derive(serde::Deserialize, Debug)]
pub struct RequestPaymentStatusParams {
    payment_status: i32,
}

#[derive(serde::Deserialize)]
pub struct RequestOrderStatusParams {
    order_status: i32,
}

pub struct OrderController;

impl OrderController {
    // 获取订单列表
    pub async fn list_orders(
        state: State<AppState>,
        Query(params): Query<Params>,
    ) -> Result<Json<serde_json::Value>, (StatusCode, &'static str)> {
        let page = params.page.unwrap_or(1);
        let posts_per_page = params.posts_per_page.unwrap_or(5);

        let (orders, num_pages) = OrderServices::get_orders(&state.conn, page, posts_per_page)
            .await
            .map_err(|e| {
                println!("Failed to get orders: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Failed to get orders")
            })?;

        let data= ResponseData {
            status: ResponseStatus::Success,
            data: Some(json!({
                    "rows": orders,
                    "num_pages": num_pages,
                }))
            ,
            code: 200,
            message: Some("Orders retrieved successfully".to_string()),
        };
        let json_data = to_value(data).unwrap();
        Ok(Json(json!(json_data)))
    }

    // 获取用户订单列表
    pub async fn list_user_orders(
        Extension(user): Extension<UserModel>,
        state: State<AppState>,
        Query(params): Query<Params>,
    ) -> Result<Json<serde_json::Value>, (StatusCode, &'static str)> {
        let orders = OrderServices::get_orders_by_user_id(&state.conn, user.id)
            .await
            .map_err(|e| {
                println!("Failed to get user orders: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Failed to get user orders")
            })?;

        let data = ResponseData {
            status: ResponseStatus::Success,
            data: 
                Some(json!({
                    "rows": orders,
                    "page": params.page.unwrap_or(1),
                    "posts_per_page": params.posts_per_page.unwrap_or(10),
                }))
            ,
            code: 200,
            message: Some("User orders retrieved successfully".to_string()),
        };
        let json_data = to_value(data).unwrap();
        println!("Json data: {:?}", json_data);
        Ok(Json(json!(json_data)))
    }

    // 获取订单详情
    pub async fn get_order(
        state: State<AppState>,
        Path(id): Path<i32>,
    ) -> Result<Json<serde_json::Value>, (StatusCode, &'static str)> {
        let order = OrderServices::get_order_by_id(&state.conn, id)
            .await
            .map_err(|e| {
                println!("Failed to get order: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Failed to get order")
            })?;

        let order_items = OrderItemServices::get_order_items_by_order_id(&state.conn, id)
            .await
            .map_err(|e| {
                println!("Failed to get order items: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Failed to get order items")
            })?;

        let data = ResponseData {
            status: ResponseStatus::Success,
            data: Some(json!({
                    "order": order,
                    "items": order_items,
                }))
            ,
            code: 200,
            message: Some("Order retrieved successfully".to_string()),
        };
        let json_data = to_value(data).unwrap();
        Ok(Json(json!(json_data)))
    }

    // 创建订单
    pub async fn create_order(
        Extension(user): Extension<entity::users::Model>,
        state: State<AppState>,
        Json(payload): Json<RequestData>,
    ) -> Result<Json<serde_json::Value>, (StatusCode, &'static str)> {
        println!("Payload: {:?}", payload);
        let order_data = OrderModel {
            user_id: user.id.clone(),
            total_price: payload.total_price.clone(),
            coupon_code: payload.coupon_code.clone(),
            gift_card_code: payload.gift_card_code.clone(),
            notes: payload.notes.clone(),
            status: 0,
            shipping_status: 0,
            shipping_address: payload.shipping_address.clone(),
            billing_address: payload.billing_address.clone(),
            payment_status: 0,
            payment_method: payload.payment_method.clone(),
            shipping_company: Some(payload.shipping_company.clone()),
            discount: payload.discount,
            tracking_number: Some(payload.tracking_number.clone()),
        };
        let order = OrderServices::create_order(&state.conn, order_data)
            .await
            .map_err(|e| {
                println!("Failed to create order: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create order")
            })?;

        // Extract order_id before using order.id multiple times
        let order_id = order.id.unwrap();

        let form_data = OrderItemModel {
            order_id: order_id,
            product_id: payload.product_id.unwrap(),
            quantity: payload.quantity.unwrap(),
            price: payload.price.unwrap(),
        };

        OrderItemServices::create_order_item(&state.conn, form_data)
            .await
            .map_err(|e| {
                println!("Failed to create order item: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to create order item",
                )
            })?;

        let data = ResponseData {
            status: ResponseStatus::Success,
            data: Some(json!(order_id)),
            code: 201,
            message: Some("Order created successfully".to_string()),
        };
        let json_data = to_value(data).unwrap();
        println!("Json data: {:?}", json_data);
        Ok(Json(json!(json_data)))
    }

    // 更新订单状态
    pub async fn update_order_status(
        state: State<AppState>,
        Path(id): Path<i32>,
        Json(payload): Json<RequestOrderStatusParams>,
    ) -> Result<Json<serde_json::Value>, (StatusCode, &'static str)> {
        OrderServices::set_order_status(&state.conn, id, payload.order_status)
            .await
            .map_err(|e| {
                println!("Failed to update order status: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to update order status",
                )
            })?;

        let data = ResponseData::<Option<serde_json::Value>> {
            status: ResponseStatus::Success,
            data: None,
            code: 200,
            message: Some("Order status updated successfully".to_string()),
        };
        let json_data = to_value(data).unwrap();
        println!("Json data: {:?}", json_data);
        Ok(Json(json!(json_data)))
    }

    // 更新支付状态
    pub async fn set_payment_status(
        state: State<AppState>,
        Path(id): Path<i32>,
        Json(payload): Json<RequestPaymentStatusParams>,
    ) -> Result<Json<serde_json::Value>, (StatusCode, &'static str)> {
        OrderServices::set_payment_status(&state.conn, id, payload.payment_status)
            .await
            .map_err(|e| {
                println!("Failed to update payment status: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to update payment status",
                )
            })?;

        let data = ResponseData::<Option<serde_json::Value>> {
            status: ResponseStatus::Success,
            data: None,
            code: 200,
            message: Some("Payment status updated successfully".to_string()),
        };
        let json_data = to_value(data).unwrap();
        Ok(Json(json!(json_data)))
    }

    // 取消订单
    pub async fn cancel_order(
        state: State<AppState>,
        Path(id): Path<i32>,
    ) -> Result<Json<serde_json::Value>, (StatusCode, &'static str)> {
        OrderServices::set_order_status(&state.conn, id, 4)
            .await
            .map_err(|e| {
                println!("Failed to cancel order: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to cancel order",
                )
            })?;

        let data = ResponseData::<Option<serde_json::Value>> {
            status: ResponseStatus::Success,
            data: None,
            code: 200,
            message: Some("Order cancelled successfully".to_string()),
        };
        let json_data = to_value(data).unwrap();
        Ok(Json(json!(json_data)))
    }
}
