use crate::tools::AppState;
use axum::{
    extract::{ State},
    http::StatusCode,
    response::Json,
};

use chrono::Date;
use serde::{Deserialize, Serialize};
use service::{sea_orm::prelude::Decimal, PaymentModel, PaymentServices};

use serde_json::json;
use serde_json::to_value;

use wechat_pay_rust_sdk::model::{AmountInfo, NativeParams};
use wechat_pay_rust_sdk::pay::WechatPay;

use alipay_sdk_rust::biz::{self, BizContenter};
use alipay_sdk_rust::pay::{PayClient, Payer};
use alipay_sdk_rust::response::TradeCreateResponse;

pub struct PaymentController;

impl PaymentController {
    pub async fn create_payment(
        state: State<AppState>,
        Json(payload): Json<RequestPaymentParams>,
    ) -> Result<Json<serde_json::Value>, (StatusCode, &'static str)> {

        let mut res_data: Option<_> = None;
        

        if payload.payment_method == 0 {

        }else if payload.payment_method == 1 {
            let out_trade_no = chrono::Utc::now().timestamp_nanos().to_string();
            let mut biz_content = biz::TradeCreateBiz::new();
            biz_content.set_subject("huawei Mate50".into());
            biz_content.set_out_trade_no(out_trade_no.clone().into()); // "1620630871769533112"
            biz_content.set_total_amount("5".into());
            biz_content.set("seller_id", "2088721038897364".into());
            biz_content.set_buyer_id("2088722038897372".into());
            use chrono::Utc;
            biz_content.set("Timestamp", Utc::now().to_string().into());
            let modal = AliPaySDKModel {
                app_id: "your_app_id".to_string(),
                alipay_root_cert_sn: "your_alipay_root_cert_sn".to_string(),
                alipay_public_key: "your_alipay_public_key".to_string(),
                app_cert_sn: "your_app_cert_sn".to_string(),
                private_key: "your_private_key".to_string(),
                public_key: "your_public_key".to_string(),
            };
            let client = Self::alipay_client(modal).await.map_err(|_| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to create Alipay client",
                )
            })?;
            let res = client.trade_create(&biz_content).map_err(|_| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to create trade with Alipay",
                )
            })?;
            println!("Alipay response: {:?}", res);
            res_data = Some(res);
        }

        
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
                println!("Failed to create product: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to create product",
                )
            })?;

        Ok(Json(json!({
            "status": "success",
            "message": "Payment created successfully",
            "data": res_data
        })))
    }

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
pub struct RequestPaymentParams{
    pub order_id: i32,
    pub payment_method: i32,
    pub amount: Decimal
}