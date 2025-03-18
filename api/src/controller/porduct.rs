
use crate::tools::{AppState, Params, ResponseData, ResponseStatus};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
};

use service::{
    sea_orm::prelude::Decimal, PorductModel, PorductServices, ProductCategoryModel,
    ProductCategoryServices,
};

use serde_json::json;
use serde_json::to_value;

#[derive(Debug, serde::Deserialize)]
pub struct PostProductModal {
    pub name: String,
    pub status: i32,
    pub category_id: i32,
    pub description: Option<String>,
    pub stock_quantity: i32,
    pub price: Decimal,
    pub image_url: Option<String>,
}

#[derive(Debug, serde::Serialize)]
pub struct ProductResponse {
    pub id: i32,
    pub name: String,
    pub status: i32,
    pub category_id: Option<i32>,
    pub description: Option<String>,
    pub stock_quantity: i32,
    pub price: Decimal,
    pub image_url: Option<String>,
}

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

        let mut porducts: Vec<ProductResponse> = porducts
            .into_iter()
            .map(|porduct| ProductResponse {
                id: porduct.id,
                name: porduct.name,
                status: porduct.status,
                category_id: None,
                description: porduct.description,
                stock_quantity: porduct.stock_quantity,
                price: porduct.price,
                image_url: porduct.image_url,
            })
            .collect();

        // 循环porducts增加category_id
        for porduct in porducts.iter_mut() {
            let product_category =
                ProductCategoryServices::find_by_product_id(&state.conn, porduct.id)
                    .await
                    .expect("Cannot find product category");

            porduct.category_id = Some(product_category.first().unwrap().category_id);
        }

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
        Json(payload): Json<PostProductModal>,
    ) -> Result<Json<serde_json::Value>, (StatusCode, &'static str)> {
        let product_data = PorductModel {
            name: payload.name.clone(),
            status: payload.status,
            description: payload.description.clone(),
            stock_quantity: payload.stock_quantity,
            price: payload.price,
            image_url: payload.image_url.clone(),
        };
        let product_res = PorductServices::create_porduct(&state.conn, product_data)
            .await
            .map_err(|e| {
                println!("Failed to create porduct: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to create porduct",
                )
            })?;

        let product_cate_data = ProductCategoryModel {
            product_id: product_res.id.unwrap(),
            category_id: payload.category_id,
        };

        let _ =
            ProductCategoryServices::create_product_category(&state.conn, product_cate_data).await;

        Ok(Json(json!({
            "status": "success",
            "message": "Porduct created successfully"
        })))
    }

    pub async fn update_porduct(
        state: State<AppState>,
        Path(id): Path<i32>,
        Json(payload): Json<PostProductModal>,
    ) -> Result<Json<serde_json::Value>, (StatusCode, &'static str)> {
        let porduct = PorductServices::get_porduct_by_id(&state.conn, id)
            .await
            .map_err(|e| {
                println!("Failed to find porduct: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Failed to find porduct")
            })?;

        let product_data = PorductModel {
            name: payload.name.clone(),
            status: payload.status,
            description: payload.description.clone(),
            stock_quantity: payload.stock_quantity,
            price: payload.price,
            image_url: payload.image_url.clone(),
        };
        PorductServices::update_porduct_by_id(&state.conn, id, product_data)
            .await
            .map_err(|e| {
                println!("Failed to update porduct: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to update porduct",
                )
            })?;

        let product_cate_data = ProductCategoryModel {
            product_id: id,
            category_id: payload.category_id,
        };

        let _ = ProductCategoryServices::update_product_category_by_product_id(
            &state.conn,
            id,
            product_cate_data,
        )
        .await;

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

        let _ =
            ProductCategoryServices::delete_product_category_by_product_id(&state.conn, id).await;

        Ok(Json(json!({
            "status": "success",
            "message": "Porduct deleted"
        })))
    }
}
