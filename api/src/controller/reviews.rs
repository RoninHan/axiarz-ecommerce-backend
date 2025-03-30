use crate::tools::{AppState, Params, ResponseData, ResponseStatus};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
};
use serde_json::json;
use serde_json::to_value;
use service::{ReviewModel, ReviewServices};

pub struct ReviewController;

impl ReviewController {
    pub async fn create_review(
        state: State<AppState>,
        Json(payload): Json<ReviewModel>,
    ) -> Result<Json<serde_json::Value>, (StatusCode, &'static str)> {
        println!("Payload: {:?}", payload);
        ReviewServices::create_review(&state.conn, payload)
            .await
            .map_err(|e| {
                println!("Failed to create review: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create review")
            })?;

        Ok(Json(json!({
            "status": "success",
            "message": "Review created successfully"
        })))
    }

    pub async fn update_review_by_id(
        Path(id): Path<i32>,
        state: State<AppState>,
        Json(payload): Json<ReviewModel>,
    ) -> Result<Json<serde_json::Value>, (StatusCode, &'static str)> {
        println!("Payload: {:?}", payload);
        ReviewServices::update_review_by_id(&state.conn, id, payload)
            .await
            .map_err(|e| {
                println!("Failed to update review: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Failed to update review")
            })?;

        Ok(Json(json!({
            "status": "success",
            "message": "Review updated successfully"
        })))
    }

    // 分页获取
    pub async fn list_reviews(
        state: State<AppState>,
        Query(params): Query<Params>,
    ) -> Result<Json<serde_json::Value>, (StatusCode, &'static str)> {
        let reviews = ReviewServices::get_reviews(
            &state.conn,
            params.page.unwrap_or(1),
            params.posts_per_page.unwrap_or(10),
        )
        .await
        .map_err(|e| {
            println!("Failed to get reviews: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to get reviews")
        })?;

        let data = ResponseData {
            status: ResponseStatus::Success,
            data: {
                json!({
                    "reviews": reviews,
                })
            },
        };

        let json_data = to_value(data).unwrap();
        println!("Json data: {:?}", json_data);

        Ok(Json(json!(json_data)))
    }

    // 删除评论
    pub async fn delete_review_by_id(
        Path(id): Path<i32>,
        state: State<AppState>,
    ) -> Result<Json<serde_json::Value>, (StatusCode, &'static str)> {
        ReviewServices::delete_review_by_id(&state.conn, id)
            .await
            .map_err(|e| {
                println!("Failed to delete review: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Failed to delete review")
            })?;

        Ok(Json(json!({
            "status": "success",
            "message": "Review deleted successfully"
        })))
    }

    // 根据产品 ID 获取评论
    pub async fn get_reviews_by_product_id(
        Path(product_id): Path<i32>,
        state: State<AppState>,
    ) -> Result<Json<serde_json::Value>, (StatusCode, &'static str)> {
        let reviews = ReviewServices::get_reviews_by_product_id(&state.conn, product_id)
            .await
            .map_err(|e| {
                println!("Failed to get reviews: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Failed to get reviews")
            })?;

        let data = ResponseData {
            status: ResponseStatus::Success,
            data: {
                json!({
                    "reviews": reviews,
                })
            },
        };

        let json_data = to_value(data).unwrap();
        println!("Json data: {:?}", json_data);

        Ok(Json(json!(json_data)))
    }
}
