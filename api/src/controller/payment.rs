use crate::tools::{AppState, ResponseData, ResponseStatus};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use service::{
    sea_orm::prelude::Decimal,
    payments::{PaymentModel, PaymentServices},
};
use serde_json::json;
use serde_json::to_value;

use wechat_pay_rust_sdk::model::{AmountInfo, NativeParams};
use wechat_pay_rust_sdk::pay::WechatPay;

use alipay_sdk_rust::biz::{self, BizContenter};
use alipay_sdk_rust::pay::{PayClient, Payer};
use alipay_sdk_rust::response::TradeCreateResponse;

pub struct PaymentController;

impl PaymentController {
    // 创建支付
    pub async fn create_payment(
        state: State<AppState>,
        Json(payload): Json<RequestPaymentParams>,
    ) -> Result<Json<serde_json::Value>, (StatusCode, &'static str)> {
        let mut res_data: Option<serde_json::Value> = None;
        let out_trade_no = Utc::now().timestamp_nanos().to_string();

        match payload.payment_method {
            // 微信支付
            0 => {
                let wechat_pay = WechatPay::new(
                    "your_app_id",
                    "your_mch_id",
                    "your_api_key",
                    "your_cert_path",
                    "your_cert_key",
                    "your_serial_no",
                );
                // 将Decimal转换为分
                let amount_cents = (payload.amount.to_string().parse::<f64>().unwrap_or(0.0) * 100.0) as i32;
                let amount = AmountInfo {
                    total: amount_cents,
                };
                let params = NativeParams {
                    description: "商品描述".to_string(),
                    out_trade_no: out_trade_no.clone(),
                    amount,
                    time_expire: None,
                    attach: None,
                    goods_tag: None,
                    support_fapiao: None,
                    scene_info: None,
                    settle_info: None,
                };
                let res = wechat_pay
                    .native_pay(params)
                    .await
                    .map_err(|e| {
                        println!("Failed to create WeChat payment: {:?}", e);
                        (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            "Failed to create WeChat payment",
                        )
                    })?;
                // 将响应转换为字符串再解析为JSON
                let res_str = format!("{:?}", res);
                res_data = Some(json!({
                    "code_url": res_str,
                    "message": "WeChat payment created successfully"
                }));
            }
            // 支付宝支付
            1 => {
                let mut biz_content = biz::TradeCreateBiz::new();
                biz_content.set_subject("商品描述".into());
                biz_content.set_out_trade_no(out_trade_no.clone().into());
                biz_content.set_total_amount(payload.amount.to_string().into());
                biz_content.set("seller_id", "your_seller_id".into());
                biz_content.set("notify_url", "https://your-domain.com/notify".into());

                let modal = AliPaySDKModel {
                    app_id: "your_app_id".to_string(),
                    alipay_root_cert_sn: "your_alipay_root_cert_sn".to_string(),
                    alipay_public_key: "your_alipay_public_key".to_string(),
                    app_cert_sn: "your_app_cert_sn".to_string(),
                    private_key: "your_private_key".to_string(),
                    public_key: "your_public_key".to_string(),
                };

                let client = Self::alipay_client(modal).await.map_err(|e| {
                    println!("Failed to create Alipay client: {:?}", e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Failed to create Alipay client",
                    )
                })?;

                let res = client.trade_create(&biz_content).map_err(|e| {
                    println!("Failed to create Alipay trade: {:?}", e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Failed to create Alipay trade",
                    )
                })?;
                // 将响应转换为字符串再解析为JSON
                let res_str = format!("{:?}", res);
                res_data = Some(json!({
                    "trade_no": res_str,
                    "message": "Alipay payment created successfully"
                }));
            }
            _ => {
                return Err((
                    StatusCode::BAD_REQUEST,
                    "Unsupported payment method",
                ));
            }
        }

        let payment_data = PaymentModel {
            order_id: payload.order_id,
            payment_method: payload.payment_method,
            transaction_id: out_trade_no,
            pay_status: 0, // 待支付
            amount: payload.amount,
            paid_at: None,
        };

        PaymentServices::create_payment(&state.conn, payment_data)
            .await
            .map_err(|e| {
                println!("Failed to create payment: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to create payment",
                )
            })?;

        let data = ResponseData {
            status: ResponseStatus::Success,
            data: {
                json!({
                    "payment_data": res_data,
                    "message": "Payment created successfully"
                })
            },
        };

        let json_data = to_value(data).unwrap();
        Ok(Json(json!(json_data)))
    }

    // 查询支付状态
    pub async fn get_payment_status(
        state: State<AppState>,
        Path(id): Path<i32>,
    ) -> Result<Json<serde_json::Value>, (StatusCode, &'static str)> {
        let payment = PaymentServices::get_payment_by_id(&state.conn, id)
            .await
            .map_err(|e| {
                println!("Failed to get payment: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Failed to get payment")
            })?;

        let data = ResponseData {
            status: ResponseStatus::Success,
            data: json!(payment),
        };

        let json_data = to_value(data).unwrap();
        Ok(Json(json!(json_data)))
    }

    // 支付回调处理
    pub async fn payment_notify(
        state: State<AppState>,
        Json(payload): Json<PaymentNotifyParams>,
    ) -> Result<Json<serde_json::Value>, (StatusCode, &'static str)> {
        // 验证签名
        if !Self::verify_signature(&payload) {
            return Err((StatusCode::BAD_REQUEST, "Invalid signature"));
        }

        // 更新支付状态
        PaymentServices::update_payment_status(
            &state.conn,
            payload.order_id,
            payload.pay_status,
            Some(Utc::now()),
        )
        .await
        .map_err(|e| {
            println!("Failed to update payment status: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to update payment status",
            )
        })?;

        Ok(Json(json!({
            "status": "success",
            "message": "Payment status updated successfully"
        })))
    }

    // 获取支付记录
    pub async fn list_payments(
        state: State<AppState>,
        Query(params): Query<PaymentQueryParams>,
    ) -> Result<Json<serde_json::Value>, (StatusCode, &'static str)> {
        let payments = PaymentServices::get_payments_by_order_id(&state.conn, params.order_id)
            .await
            .map_err(|e| {
                println!("Failed to get payments: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Failed to get payments")
            })?;

        let data = ResponseData {
            status: ResponseStatus::Success,
            data: json!(payments),
        };

        let json_data = to_value(data).unwrap();
        Ok(Json(json!(json_data)))
    }

    // 支付宝客户端配置
    async fn alipay_client(modal: AliPaySDKModel) -> Result<impl Payer, anyhow::Error> {
        let client = PayClient::builder()
            .api_url("https://openapi-sandbox.dl.alipaydev.com/gateway.do")
            .app_id(&modal.app_id)
            .alipay_root_cert_sn(&modal.alipay_root_cert_sn)
            .alipay_public_key(&modal.alipay_public_key)
            .app_cert_sn(&modal.app_cert_sn)
            .charset_utf8()
            .format_json()
            .private_key(&modal.private_key)
            .public_key(&modal.public_key)
            .sign_type_rsa2()
            .version_1_0()
            .build()?;
        Ok(client)
    }

    // 验证签名
    fn verify_signature(payload: &PaymentNotifyParams) -> bool {
        // 实现签名验证逻辑
        true
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AliPaySDKModel {
    pub app_id: String,
    pub alipay_root_cert_sn: String,
    pub alipay_public_key: String,
    pub app_cert_sn: String,
    pub private_key: String,
    pub public_key: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RequestPaymentParams {
    pub order_id: i32,
    pub payment_method: i32,
    pub amount: Decimal,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PaymentNotifyParams {
    pub order_id: i32,
    pub pay_status: i32,
    pub signature: String,
    pub timestamp: String,
    pub nonce: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PaymentQueryParams {
    pub order_id: i32,
}