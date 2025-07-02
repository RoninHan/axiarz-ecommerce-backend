use crate::{
    middleware::auth::Auth,
    tools::{AppState, ResponseData, ResponseStatus},
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    Extension,
};
use entity::users::Model as UserModel;
use serde_json::json;
use serde_json::to_value;
use service::invoice::{InvoiceModel, InvoiceServices};

pub struct InvoiceController;

impl InvoiceController {
    /// 创建发票信息
    pub async fn create_invoice(
        Extension(user): Extension<UserModel>,
        state: State<AppState>,
        Json(payload): Json<InvoiceModel>,
    ) -> Result<Json<serde_json::Value>, (StatusCode, &'static str)> {
        // 确保用户只能创建自己的发票信息

        let payload = InvoiceModel {
            user_id: user.id,
            ..payload
        };

        InvoiceServices::create_invoice(&state.conn, payload)
            .await
            .map_err(|e| {
                println!("Failed to create invoice: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create invoice")
            })?;

        let data = ResponseData {
            status: ResponseStatus::Success,
            data: json!({
                "message": "Invoice created successfully"
            }),
        };

        let json_data = to_value(data).unwrap();
        Ok(Json(json!(json_data)))
    }

    /// 更新发票信息
    pub async fn update_invoice(
        Extension(user): Extension<UserModel>,
        state: State<AppState>,
        Path(id): Path<i32>,
        Json(payload): Json<InvoiceModel>,
    ) -> Result<Json<serde_json::Value>, (StatusCode, &'static str)> {

        let payload = InvoiceModel {
            user_id: user.id,
            ..payload
        };

        InvoiceServices::update_invoice_by_id(&state.conn, id, payload)
            .await
            .map_err(|e| {
                println!("Failed to update invoice: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Failed to update invoice")
            })?;

        let data = ResponseData {
            status: ResponseStatus::Success,
            data: json!({
                "message": "Invoice updated successfully"
            }),
        };

        let json_data = to_value(data).unwrap();
        Ok(Json(json!(json_data)))
    }

    /// 删除发票信息
    pub async fn delete_invoice(
        Extension(user): Extension<UserModel>,
        state: State<AppState>,
        Path(id): Path<i32>,
    ) -> Result<Json<serde_json::Value>, (StatusCode, &'static str)> {
        // 先获取发票信息，确保用户只能删除自己的发票
        let invoice = InvoiceServices::get_invoice_by_id(&state.conn, id)
            .await
            .map_err(|e| {
                println!("Failed to get invoice: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Failed to get invoice")
            })?;

        if let Some(invoice) = invoice {
            if invoice.user_id != user.id {
                return Err((StatusCode::FORBIDDEN, "Cannot delete invoice for other users"));
            }
        } else {
            return Err((StatusCode::NOT_FOUND, "Invoice not found"));
        }

        InvoiceServices::delete_invoice_by_id(&state.conn, id)
            .await
            .map_err(|e| {
                println!("Failed to delete invoice: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Failed to delete invoice")
            })?;

        let data = ResponseData {
            status: ResponseStatus::Success,
            data: json!({
                "message": "Invoice deleted successfully"
            }),
        };

        let json_data = to_value(data).unwrap();
        Ok(Json(json!(json_data)))
    }

    /// 获取用户的所有发票信息
    pub async fn get_invoices(
        Extension(user): Extension<UserModel>,
        state: State<AppState>,
    ) -> Result<Json<serde_json::Value>, (StatusCode, &'static str)> {
        let invoices = InvoiceServices::get_invoices_by_user_id(&state.conn, user.id)
            .await
            .map_err(|e| {
                println!("Failed to get invoices: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Failed to get invoices")
            })?;

        let data = ResponseData {
            status: ResponseStatus::Success,
            data: json!({
                "invoices": invoices
            }),
        };

        let json_data = to_value(data).unwrap();
        Ok(Json(json!(json_data)))
    }

    /// 获取单个发票信息
    pub async fn get_invoice(
        Extension(user): Extension<UserModel>,
        state: State<AppState>,
        Path(id): Path<i32>,
    ) -> Result<Json<serde_json::Value>, (StatusCode, &'static str)> {
        let invoice = InvoiceServices::get_invoice_by_id(&state.conn, id)
            .await
            .map_err(|e| {
                println!("Failed to get invoice: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Failed to get invoice")
            })?;

        if let Some(invoice) = invoice {
            // 确保用户只能查看自己的发票信息
            if invoice.user_id != user.id {
                return Err((StatusCode::FORBIDDEN, "Cannot view invoice for other users"));
            }

            let data = ResponseData {
                status: ResponseStatus::Success,
                data: json!({
                    "invoice": invoice
                }),
            };

            let json_data = to_value(data).unwrap();
            Ok(Json(json!(json_data)))
        } else {
            Err((StatusCode::NOT_FOUND, "Invoice not found"))
        }
    }

    /// 获取用户的默认发票信息
    pub async fn get_default_invoice(
        Extension(user): Extension<UserModel>,
        state: State<AppState>,
    ) -> Result<Json<serde_json::Value>, (StatusCode, &'static str)> {
        let invoice = InvoiceServices::get_default_invoice_by_user_id(&state.conn, user.id)
            .await
            .map_err(|e| {
                println!("Failed to get default invoice: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Failed to get default invoice")
            })?;

        let data = ResponseData {
            status: ResponseStatus::Success,
            data: json!({
                "invoice": invoice
            }),
        };

        let json_data = to_value(data).unwrap();
        Ok(Json(json!(json_data)))
    }
} 