use std::{default, result};

use crate::tools::{AppState, Params, ResponseData, ResponseStatus};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
};

use entity::categories;
use service::{PaymentModel, PaymentServices, PorductServices, RequestCreatePaymentBody};

use serde_json::json;
use serde_json::to_value;

use wechat_pay_rust_sdk::model::{AmountInfo, NativeParams};
use wechat_pay_rust_sdk::pay::WechatPay;

pub struct PaymentController;

impl PaymentController {
    pub async fn create_payment(
        state: State<AppState>,
        Json(payload): Json<RequestCreatePaymentBody>,
    ) -> Result<Json<serde_json::Value>, (StatusCode, &'static str)> {
        let private_key_path = "./apiclient_key.pem";
        let private_key = std::fs::read_to_string(private_key_path).unwrap();
        let wechat_pay = WechatPay::new(
            "app_id",
            "mch_id",
            private_key.as_str(),
            "serial_no",
            "v3_key",
            "notifi_url",
        );

        let body = wechat_pay
            .native_pay(NativeParams::new(
                "测试支付1分",
                "124324343",
                payload.amount.clone().into(),
            ))
            .await;

        let payment_data = PaymentModel {
            order_id: payload.order_id,
            payment_method: payload.payment_method,
            transaction_id: String::from(""),
            pay_status: 2,
            amount: payload.amount.into(),
            paid_at: None,
        };

        println!("Payload: {:?}", payload);
        PaymentServices::create_payment(&state.conn, payment_data)
            .await
            .map_err(|e| {
                println!("Failed to create porduct: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to create porduct",
                )
            })?;

        Ok(Json(json!({
            "status": "success",
            "message": "Porduct created successfully"
        })))
    }
}
