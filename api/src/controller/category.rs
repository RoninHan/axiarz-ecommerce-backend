use crate::tools::{AppState, Params, ResponseData, ResponseStatus};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
};
use service::categories::{CategoryModel, CategoryServices};
use service::product_categories::ProductCategoryServices;
use serde_json::json;
use serde_json::to_value;

pub struct CategoryController;

impl CategoryController {
    pub async fn list_categories(
        state: State<AppState>,
        Query(params): Query<Params>,
    ) -> Result<Json<serde_json::Value>, (StatusCode, &'static str)> {
        let page = params.page.unwrap_or(1);
        let posts_per_page = params.posts_per_page.unwrap_or(5);

        let (categories, num_pages) =
            CategoryServices::get_categories_by_page(&state.conn, page, posts_per_page)
                .await
                .expect("Cannot find categories in page");

        let data = ResponseData {
            status: ResponseStatus::Success,
            data: {
                json!({
                    "categories": categories,
                    "num_pages": num_pages,
                })
            },
        };

        let json_data = to_value(data).unwrap();
        println!("Json data: {:?}", json_data);
        Ok(Json(json!(json_data)))
    }

    pub async fn create_category(
        state: State<AppState>,
        Json(payload): Json<CategoryModel>,
    ) -> Result<Json<serde_json::Value>, (StatusCode, &'static str)> {
        println!("Payload: {:?}", payload);
        CategoryServices::create_category(&state.conn, payload)
            .await
            .map_err(|e| {
                println!("Failed to create category: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to create category",
                )
            })?;

        Ok(Json(json!({
            "status": "success",
            "message": "Category created successfully"
        })))
    }

    pub async fn update_category(
        state: State<AppState>,
        Path(id): Path<i32>,
        Json(payload): Json<CategoryModel>,
    ) -> Result<Json<serde_json::Value>, (StatusCode, &'static str)> {
        println!("Payload: {:?}", payload);
        CategoryServices::update_category_by_id(&state.conn, id, payload)
            .await
            .map_err(|e| {
                println!("Failed to update category: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to update category",
                )
            })?;

        Ok(Json(json!({
            "status": "success",
            "message": "Category updated successfully"
        })))
    }

    pub async fn delete_category(
        state: State<AppState>,
        Path(id): Path<i32>,
    ) -> Result<Json<serde_json::Value>, (StatusCode, &'static str)> {
        // 查询product_category表
        let product_category_data =
            ProductCategoryServices::find_by_category_id(&state.conn, id).await;

        // 如果有数据，则提示：删除失败，需要先修改对应商品类别，再尝试删除
        if let Ok(data) = product_category_data {
            if data.len() > 0 {
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to delete category, please update the corresponding product category first",
                ));
            }
        }

        // 再删除category表中的数据
        CategoryServices::delete_category_by_id(&state.conn, id)
            .await
            .map_err(|e| {
                println!("Failed to delete category: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to delete category",
                )
            })?;

        Ok(Json(json!({
            "status": "success",
            "message": "Category deleted successfully"
        })))
    }
}
