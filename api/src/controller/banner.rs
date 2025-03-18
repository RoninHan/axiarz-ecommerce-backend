use crate::tools::{AppState, Params, ResponseData, ResponseStatus};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
};
use serde_json::json;
use serde_json::to_value;
use service::{BannerModel, BannerServices};

pub struct BannerController;

impl BannerController {
    pub async fn list_banners_all(
        state: State<AppState>,
        Query(params): Query<Params>,
    ) -> Result<Json<serde_json::Value>, (StatusCode, &'static str)> {
        let banners = BannerServices::get_banner_all(&state.conn)
            .await
            .map_err(|e| {
                println!("Failed to get banners: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to get banners",
                )
            })?;

        let data = ResponseData {
            status: ResponseStatus::Success,
            data: {
                json!({
                    "banners": banners,
                })
            },
        };

        let json_data = to_value(data).unwrap();
        println!("Json data: {:?}", json_data);

        Ok(Json(json!(json_data)))
    }

    pub async fn create_banner(
        state: State<AppState>,
        Json(payload): Json<BannerModel>,
    ) -> Result<Json<serde_json::Value>, (StatusCode, &'static str)> {
        println!("Payload: {:?}", payload);
        BannerServices::create_banner(&state.conn, payload)
            .await
            .map_err(|e| {
                println!("Failed to create banner: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to create banner",
                )
            })?;

        Ok(Json(json!({
            "status": "success",
            "message": "Banner created successfully"
        })))
    }

    pub async fn update_banner(
        state: State<AppState>,
        Path(id): Path<i32>,
        Json(payload): Json<BannerModel>,
    ) -> Result<Json<serde_json::Value>, (StatusCode, &'static str)> {
        println!("Payload: {:?}", payload);
        BannerServices::update_banner_by_id(&state.conn, id, payload)
            .await
            .map_err(|e| {
                println!("Failed to update banner: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to update banner",
                )
            })?;

        Ok(Json(json!({
            "status": "success",
            "message": "Banner updated successfully"
        })))
    }

    pub async fn delete_banner(
        state: State<AppState>,
        Path(id): Path<i32>,
    ) -> Result<Json<serde_json::Value>, (StatusCode, &'static str)> {
        BannerServices::delete_banner_by_id(&state.conn, id)
            .await
            .map_err(|e| {
                println!("Failed to delete banner: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to delete banner",
                )
            })?;

        Ok(Json(json!({
            "status": "success",
            "message": "Banner deleted successfully"
        })))
    }
}