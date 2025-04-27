

use crate::tools::{AppState, Params, ResponseData, ResponseStatus};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    Extension,
};
use entity::{ users::Model as UserModel};
use serde_json::json;
use serde_json::to_value;
use service::{
    sea_orm::prelude::Decimal, OrderItemModel, OrderItemServices, OrderModel, OrderServices,
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
            .expect("Cannot find posts in page");

        let data = ResponseData {
            status: ResponseStatus::Success,
            data: {
                json!({
                    "orders": orders,
                    "num_pages": num_pages,
                })
            },
        };

        let json_data = to_value(data).unwrap();
        println!("Json data: {:?}", json_data);
        Ok(Json(json!(json_data)))
    }

    // 第一步创建订单
    pub async fn create_order(
        Extension(user): Extension<UserModel>,
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

        let form_data = OrderItemModel {
            order_id: order.id.unwrap(),
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

        Ok(Json(json!({
            "status": "success",
            "message": "Order created successfully"
        })))
    }

    pub async fn set_payment_status(
        state: State<AppState>,
        Path(id): Path<i32>,
        Json(payload): Json<RequestPaymentStatusParams>,
    ) -> Result<Json<serde_json::Value>, (StatusCode, &'static str)> {
        OrderServices::set_payment_status(&state.conn, id, payload.payment_status.clone())
            .await
            .map_err(|e| {
                println!("Failed to update porduct: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to update porduct",
                )
            })?;

        Ok(Json(json!({
            "status": "success",
            "message": "Porduct updated"
        })))
    }

    pub async fn update_order_status(
        state: State<AppState>,
        Path(id): Path<i32>,
        Json(payload): Json<RequestOrderStatusParams>,
    ) -> Result<Json<serde_json::Value>, (StatusCode, &'static str)> {
        OrderServices::set_order_status(&state.conn, id, payload.order_status.clone())
            .await
            .map_err(|e| {
                println!("Failed to update porduct: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to update porduct",
                )
            })?;

        Ok(Json(json!({
            "status": "success",
            "message": "Porduct updated"
        })))
    }

    // 取消订单
    pub async fn cancel_order(
        state: State<AppState>,
        Path(id): Path<i32>,
    ) -> Result<Json<serde_json::Value>, (StatusCode, &'static str)> {
        OrderServices::set_order_status(&state.conn, id, 4)
            .await
            .map_err(|e| {
                println!("Failed to update porduct: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to update porduct",
                )
            })?;

        Ok(Json(json!({
            "status": "success",
            "message": "Porduct updated"
        })))
    }
}
