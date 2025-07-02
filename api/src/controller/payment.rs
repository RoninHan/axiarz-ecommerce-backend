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
                let mut biz_content = biz::TradePagePayBiz::new();
                biz_content.set_subject("huawei Mate50".into());
                biz_content.set_out_trade_no(out_trade_no.clone().into());
                biz_content.set_total_amount(payload.amount.to_string().into());
                // 设置卖家ID
                biz_content.set("seller_id", "2088721067342945".into()); //2088841836934905
                // 设置回调地址
                biz_content.set("notify_url", "https://www.axiarz.com/api/notify".into());

                biz_content.set("Timestamp", "2025-05-10 23:52:04".into());

                let alipay_root_cert_sn_path = "./sandbox/alipayRootCert.crt";
                let alipay_public_key_path = "./sandbox/alipayPublicCert.crt";
                let app_cert_sn_path = "./sandbox/appPublicCert.crt";

                let app_cert_sn = alipay_sdk_rust::cert::get_cert_sn(app_cert_sn_path).unwrap();
                let alipay_public_key = alipay_sdk_rust::cert::get_public_key_with_path(alipay_public_key_path).unwrap();
                let alipay_root_cert_sn = alipay_sdk_rust::cert::get_root_cert_sn(alipay_root_cert_sn_path).unwrap();

                println!("app_cert_sn: {:?}", app_cert_sn);
                println!("alipay_public_key: {:?}", alipay_public_key);
                println!("alipay_root_cert_sn: {:?}", alipay_root_cert_sn);

                let modal = AliPaySDKModel {
                    app_id: "2021000148665299".to_string(),//2021005148613160
                    alipay_root_cert_sn: alipay_root_cert_sn,
                    alipay_public_key: alipay_public_key,
                    app_cert_sn: app_cert_sn,
                    private_key: "MIIEpAIBAAKCAQEAopEI5NfIvBUN38gZsregDbEEYGJuN9zVKSI3SGbkOlN+oD9xKV1eZYR8rvLEcARAQned/UrcSixje4nMDW22BlEOoAzneNZT3Eka9Y6ze5P2OXw87haKlC1AEKXxBYTQ9RT0XB4znga52eojU2dZoHM/HkPwlg1POLbBwDnEdMsf4/7VtGgqzVKtrL5sTKPpkh18hBujKhyF4sZol865p0eDmM8GQHKMBJUe8yj0kzq22lVlHYualPIlIJOEI5rS8kQTUim2ZHVAcIXz0QzMVfu7ZP85vfST0LDKrUZ+KyGkVod6YLKVl+SwzX41Kua1vNNyWnwvFRH6/iCUpMChywIDAQABAoIBAGbzED3UBVROxQWFs/iA3wQsqQfc7c3EtN0ixP294ySowZT7+E7oySHjAA7OwOXrW0J8e/nvEYiLicivYCDU7KQdavSil2fn2x2y0jbV0wYckp6e9fsVHVdvPJYOcI89KBM83O8FVUzrF4FQDGUCGzlIIp2pCtALx7Kz0glIzAflpDqL62GsO0gtTsxsY+LLyq3ch/3sJCKxJrUxGLyUmQKepcV/PEOYC0+ELg2TYeDpW4B5PUPWCMWd3GzQLLMXR2xdYr/HPEq81eXGjid7xCrDOfRO9qDTdyn+ojL32NzgPU9n8p8e3nWQj7AtgnUeRcxfEGFrpz01qJ0YvD3IsmECgYEA1YQmSxp7iNFNDRIcEte8HyePztaCVnl5AE61HvUn01rPuUCfpmLJLtXDmw95m6veJHSetpX8yI2qRr5BlZ4LsCyndcWRhmc1nYoCVH6eImEv0dy/VseOCce/F9sffx1Tw8zsdVcd4TOeQ9AcFR5slR9ROSAsxrXKlSuWWV4KYBECgYEAwumq+JIqxybU/0gmxfLQNOTyw6cYwNniSYdQFjmkV6Hk0uL2yE6vsXS4TVhimXCrzNrLWmT4hYg/knlDHMtMGKoVivgv3rRaBrkpnB1BqA8wHepq3GtX6hVxNPqqO4qRfsLNS/co1lbF0Q1pzpAEJfDOJ+3Xqn2CkIIRLY+ggBsCgYEAhqoAVOHxBAut4w6G8kNqfOPAyZ11OwEGFfGOPmY0phLibF6b0p7/cDCWXBfYhRxS0P9UkqfqdLsUp+WbC7hQet7PB3KCJBi4MhI3Af+R0PEm7d+iNiNKLFT06yDirpNan2WBxUgaqkyaBv8clx1HMo479iGa3AEQMiD5hIfRV2ECgYAUa4qf9CBLMQRLonF9d4zcncfZO55aRflxHp4DVhKjo7Bnb6PPJH8/pizQ3Zum26kEE0AOvllTFA0k+VNQpvPX+am8H3hUaqyr26ZCVsZUJxMxbye25AAX5BsyI7jF+CR6FUqQ1NoQapLa7f7Rx0DIAMx4XnCjyfZt0VKfZVa5VQKBgQCXYVPSPQzZER7Pm5bX2I8MhYpfnerq2MKEHmTkSYRw4M4LxXgN8U4In9oBQMv4dFqdl8g6f3Hzvr97fUmZOF/WAcMmefdCGe5Kuv257w1E5aGG/UGm3LR7ptXoCntD5nBxGSvxJySsBSY4q/9Xe4yQ1927q2tYTYlxXE2+tCrIng==".to_string(),
                    public_key: "MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAugzTWWmvJuOJ/f/sPXM2YiPs3753Ehx+SQ/j4fA74TI1unczQsfS2gVaV31ueMv+tE+Bv86os5hd5KjS0Vxu0RgWsbR7tUCgMJlLgHcOyzldNlaO/FV7tJqFFdClSAtBPXvcz0inDNFW9e4Wme/4phDNCx0kSgWVRjmSF+69DjE35YIMW5qT1bOD+tzAojXG2j65qlO69OGxh/J6Dea2+oLXoLfwfWxP7rizffiO5e4MgGMRkwPsMLZ6izyRs6Eaz80YMghb4zAxFlBok9m+RvjewivAawL4Nsjmg1K3gfHuxv89tb/GJYIJwd50w75oHuq3yPntR4jJ80AST9JGUwIDAQAB".to_string(),
                };

                let client = Self::alipay_client(modal).await.map_err(|e| {
                    println!("Failed to create Alipay client: {:?}", e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Failed to create Alipay client",
                    )
                })?;
                println!("client: {:?}", "123123");
                println!("biz_content: {:?}", serde_json::to_string(&biz_content).unwrap());
                let res = client.trade_page_pay(&biz_content).map_err(|e| {
                    println!("Failed to create Alipay trade: {:?}", e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Failed to create Alipay trade",
                    )
                })?;
                println!("123111111");
                println!("res: {:?}", res);
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