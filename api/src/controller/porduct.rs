use crate::{
    middleware::auth::Auth,
    tools::{AppState, Params, ResponseData, ResponseStatus},
};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
};

use service::{PorductModel, PorductServices};

use serde_json::json;
use serde_json::to_value;

pub struct PorductController;

impl PorductController {
    pub async fn list_porducts(
        state: State<AppState>,
        Query(params): Query<Params>,
    ) -> Result<Json<serde_json::Value>, (StatusCode, &'static str)> {
        let page = params.page.unwrap_or(1);
        let posts_per_page = params.posts_per_page.unwrap_or(5);

        let (porducts, num_pages) =
            PorductServices::get_porducts_by_page(&state.conn, page, posts_per_page)
                .await
                .expect("Cannot find posts in page");

        let data = ResponseData {
            status: ResponseStatus::Success,
            data: {
                json!({
                    "porducts": porducts,
                    "num_pages": num_pages,
                })
            },
        };

        let json_data = to_value(data).unwrap();
        println!("Json data: {:?}", json_data);
        Ok(Json(json!(json_data)))
    }

    pub async fn create_porduct(
        state: State<AppState>,
        Json(payload): Json<PorductModel>,
    ) -> Result<Json<serde_json::Value>, (StatusCode, &'static str)> {
        println!("Payload: {:?}", payload);
        PorductServices::create_porduct(&state.conn, payload)
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

    pub async fn update_porduct(
        state: State<AppState>,
        Path(id): Path<i32>,
        Json(payload): Json<PorductModel>,
    ) -> Result<Json<serde_json::Value>, (StatusCode, &'static str)> {
        let porduct = PorductServices::get_porduct_by_id(&state.conn, id)
            .await
            .map_err(|e| {
                println!("Failed to find porduct: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Failed to find porduct")
            })?;

        let porduct = PorductServices::update_porduct_by_id(&state.conn, id, payload)
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

    pub async fn delete_porduct(
        state: State<AppState>,
        Path(id): Path<i32>,
    ) -> Result<Json<serde_json::Value>, (StatusCode, &'static str)> {
        PorductServices::delete_porduct_by_id(&state.conn, id)
            .await
            .map_err(|e| {
                println!("Failed to delete porduct: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to delete porduct",
                )
            })?;

        Ok(Json(json!({
            "status": "success",
            "message": "Porduct deleted"
        })))
    }
}
