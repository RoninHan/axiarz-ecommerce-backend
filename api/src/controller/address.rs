use crate::{
    middleware::auth::Auth,
    tools::{AppState, Params, ResponseData, ResponseStatus},
};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
};
use service::address::{AddressModel, AddressServices};
use serde_json::json;
use serde_json::to_value;

pub struct AddressController;

impl AddressController {
    pub async fn create_address(
        state: State<AppState>,
        Json(payload): Json<AddressModel>,
    ) -> Result<Json<serde_json::Value>, (StatusCode, &'static str)> {
        println!("Payload: {:?}", payload);
        AddressServices::create_address(&state.conn, payload)
            .await
            .expect("Cannot create address");

        let data = ResponseData {
            status: ResponseStatus::Success,
            data: json!({
                "message": "Address created successfully"
            }),
        };

        let json_data = to_value(data).unwrap();
        Ok(Json(json!(json_data)))
    }

    pub async fn update_address(
        state: State<AppState>,
        Path(id): Path<i32>,
        Json(payload): Json<AddressModel>,
    ) -> Result<Json<serde_json::Value>, (StatusCode, &'static str)> {
        println!("Payload: {:?}", payload);
        AddressServices::update_address_by_id(&state.conn, id, payload)
            .await
            .expect("Cannot update address");

        let data = ResponseData {
            status: ResponseStatus::Success,
            data: json!({
                "message": "Address updated successfully"
            }),
        };

        let json_data = to_value(data).unwrap();
        Ok(Json(json!(json_data)))
    }

    pub async fn delete_address(
        state: State<AppState>,
        Path(id): Path<i32>,
    ) -> Result<Json<serde_json::Value>, (StatusCode, &'static str)> {
        AddressServices::delete_address_by_id(&state.conn, id)
            .await
            .expect("Cannot delete address");

        let data = ResponseData {
            status: ResponseStatus::Success,
            data: json!({
                "message": "Address deleted successfully"
            }),
        };

        let json_data = to_value(data).unwrap();
        Ok(Json(json!(json_data)))
    }

    pub async fn get_addresses_by_user_id(
        state: State<AppState>,
        Path(user_id): Path<i32>,
    ) -> Result<Json<serde_json::Value>, (StatusCode, &'static str)> {
        let addresses = AddressServices::get_addresses_by_user_id(&state.conn, user_id)
            .await
            .expect("Cannot find addresses");

        let data = ResponseData {
            status: ResponseStatus::Success,
            data: json!({
                "rows": addresses
            }),
        };

        let json_data = to_value(data).unwrap();
        Ok(Json(json!(json_data)))
    }

    pub async fn get_address_by_id(
        state: State<AppState>,
        Path(id): Path<i32>,
    ) -> Result<Json<serde_json::Value>, (StatusCode, &'static str)> {
        let address = AddressServices::get_address_by_id(&state.conn, id)
            .await
            .expect("Cannot find address");

        let data = ResponseData {
            status: ResponseStatus::Success,
            data: json!({
                "address": address
            }),
        };

        let json_data = to_value(data).unwrap();
        Ok(Json(json!(json_data)))
    }
} 