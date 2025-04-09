use std::{fs, str::FromStr};

use crate::tools::{AppState, Params, ResponseData, ResponseStatus};
use axum::{
    extract::{Multipart, Path, Query, State},
    http::StatusCode,
    response::Json,
};

use entity::product_categories;
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
            println!("Product category: {:?}", product_category);
            porduct.category_id = Some(product_category.first().unwrap().category_id);
        }

        let data = ResponseData {
            status: ResponseStatus::Success,
            data: {
                json!({
                    "data": porducts,
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
        mut multipart: Multipart,
    ) -> Result<Json<serde_json::Value>, (StatusCode, &'static str)> {
        let mut product_name = None;
        let mut status = None;
        let mut description = None;
        let mut stock_quantity = None;
        let mut price = None;
        let mut image_url = None;
        let mut category_id = None;

        while let Some(field) = multipart.next_field().await.map_err(|_| {
            (
                StatusCode::BAD_REQUEST,
                "Failed to process multipart form data",
            )
        })? {
            let name = field.name().unwrap_or("").to_string();
            if name == "name" {
                let data = field.text().await.map_err(|_| {
                    (
                        StatusCode::BAD_REQUEST,
                        "Failed to read name field from form data",
                    )
                })?;
                product_name = Some(data);
            } else if name == "status" {
                let data = field.text().await.map_err(|_| {
                    (
                        StatusCode::BAD_REQUEST,
                        "Failed to read status field from form data",
                    )
                })?;
                status = Some(data);
            } else if name == "description" {
                let data = field.text().await.map_err(|_| {
                    (
                        StatusCode::BAD_REQUEST,
                        "Failed to read description field from form data",
                    )
                })?;
                description = Some(data);
            } else if name == "stock_quantity" {
                let data = field.text().await.map_err(|_| {
                    (
                        StatusCode::BAD_REQUEST,
                        "Failed to read stock_quantity field from form data",
                    )
                })?;
                stock_quantity = Some(data);
            } else if name == "price" {
                let data = field.text().await.map_err(|_| {
                    (
                        StatusCode::BAD_REQUEST,
                        "Failed to read price field from form data",
                    )
                })?;
                price = Some(data);
            } else if name == "image" {
                // 提取圖片文件
                let file_name = field.file_name().unwrap_or("default.png").to_string();
                let file_data = field
                    .bytes()
                    .await
                    .map_err(|_| (StatusCode::BAD_REQUEST, "Failed to read image file"))?;

                // 确保目标目录存在
                let upload_dir = "./uploads";
                if !std::path::Path::new(upload_dir).exists() {
                    std::fs::create_dir_all(upload_dir).map_err(|e| {
                        eprintln!("Failed to create upload directory: {:?}", e); // 打印具体的错误信息
                        (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            "Failed to create upload directory",
                        )
                    })?;
                }

                // 保存图片到服务器
                let file_path = format!("{}/{}", upload_dir, file_name);
                fs::write(&file_path, &file_data).map_err(|e| {
                    eprintln!("Failed to save image file: {:?}", e); // 打印具体的错误信息
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Failed to save image file",
                    )
                })?;
                image_url = Some(file_path.replace("./", "/"));
            } else if name == "category_id" {
                let data = field.text().await.map_err(|_| {
                    (
                        StatusCode::BAD_REQUEST,
                        "Failed to read category_id field from form data",
                    )
                })?;
                category_id = Some(data);
            }
        }

        let product_data = PorductModel {
            name: product_name.unwrap(),
            status: status.unwrap().parse().unwrap(),
            description: description,
            stock_quantity: stock_quantity.unwrap().parse().unwrap(),
            price: Decimal::from_str(price.as_ref().unwrap()).unwrap(),
            image_url: image_url,
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
            category_id: category_id.unwrap().parse().unwrap(),
        };
        println!("Product category data: {:?}", product_cate_data);
        let product_categories_data =
            ProductCategoryServices::create_product_category(&state.conn, product_cate_data)
                .await
                .map_err(|e| {
                    println!("Failed to create product category: {:?}", e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Failed to create product category",
                    )
                })?;
        println!("Product categories data: {:?}", product_categories_data);

        Ok(Json(json!({
            "status": "success",
            "message": "Porduct created successfully"
        })))
    }

    pub async fn update_porduct(
        state: State<AppState>,
        Path(id): Path<i32>,
        mut multipart: Multipart,
    ) -> Result<Json<serde_json::Value>, (StatusCode, &'static str)> {
        let mut product_name = None;
        let mut status = None;
        let mut description = None;
        let mut stock_quantity = None;
        let mut price = None;
        let mut image_url = None;
        let mut category_id = None;

        while let Some(field) = multipart.next_field().await.map_err(|_| {
            (
                StatusCode::BAD_REQUEST,
                "Failed to process multipart form data",
            )
        })? {
            let name = field.name().unwrap_or("").to_string();
            if name == "name" {
                let data = field.text().await.map_err(|_| {
                    (
                        StatusCode::BAD_REQUEST,
                        "Failed to read name field from form data",
                    )
                })?;
                product_name = Some(data);
            } else if name == "status" {
                let data = field.text().await.map_err(|_| {
                    (
                        StatusCode::BAD_REQUEST,
                        "Failed to read status field from form data",
                    )
                })?;
                status = Some(data);
            } else if name == "description" {
                let data = field.text().await.map_err(|_| {
                    (
                        StatusCode::BAD_REQUEST,
                        "Failed to read description field from form data",
                    )
                })?;
                description = Some(data);
            } else if name == "stock_quantity" {
                let data = field.text().await.map_err(|_| {
                    (
                        StatusCode::BAD_REQUEST,
                        "Failed to read stock_quantity field from form data",
                    )
                })?;
                stock_quantity = Some(data);
            } else if name == "price" {
                let data = field.text().await.map_err(|_| {
                    (
                        StatusCode::BAD_REQUEST,
                        "Failed to read price field from form data",
                    )
                })?;
                price = Some(data);
            } else if name == "image" {
                // 提取圖片文件
                let file_name = field.file_name().unwrap_or("default.png").to_string();
                let file_data = field
                    .bytes()
                    .await
                    .map_err(|_| (StatusCode::BAD_REQUEST, "Failed to read image file"))?;

                // 确保目标目录存在
                let upload_dir = "./uploads";
                if !std::path::Path::new(upload_dir).exists() {
                    std::fs::create_dir_all(upload_dir).map_err(|e| {
                        eprintln!("Failed to create upload directory: {:?}", e); // 打印具体的错误信息
                        (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            "Failed to create upload directory",
                        )
                    })?;
                }

                // 保存图片到服务器
                let file_path = format!("{}/{}", upload_dir, file_name);
                fs::write(&file_path, &file_data).map_err(|e| {
                    eprintln!("Failed to save image file: {:?}", e); // 打印具体的错误信息
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Failed to save image file",
                    )
                })?;
                image_url = Some(file_path.replace("./", "/"));
            } else if name == "category_id" {
                let data = field.text().await.map_err(|_| {
                    (
                        StatusCode::BAD_REQUEST,
                        "Failed to read category_id field from form data",
                    )
                })?;
                category_id = Some(data);
            }
        }
        println!("1111111111111111111");
        let porduct = PorductServices::get_porduct_by_id(&state.conn, id)
            .await
            .map_err(|e| {
                println!("Failed to find porduct: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Failed to find porduct")
            })?;
        println!("2222222222222222222222");
        let product_data = PorductModel {
            name: product_name.unwrap_or(porduct.name),
            status: status
                .unwrap_or(porduct.status.to_string())
                .parse()
                .unwrap(),
            description: description.or(porduct.description),
            stock_quantity: stock_quantity
                .unwrap_or(porduct.stock_quantity.to_string())
                .parse()
                .unwrap(),
            price: Decimal::from_str(&price.unwrap_or(porduct.price.to_string()).to_string())
                .unwrap(),
            image_url: image_url.or(porduct.image_url),
        };

        println!("3333333333333333333333");
        PorductServices::update_porduct_by_id(&state.conn, id, product_data)
            .await
            .map_err(|e| {
                println!("Failed to update porduct: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to update porduct",
                )
            })?;
        println!("444444444444444444444444");
        let product_cate_data = ProductCategoryModel {
            product_id: id,
            category_id: category_id.unwrap().parse().unwrap(),
        };

        let _ = ProductCategoryServices::update_product_category_by_product_id(
            &state.conn,
            id,
            product_cate_data,
        )
        .await
        .map_err(|e| {
            println!("Failed to update product category: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to update product category",
            )
        })?;
        println!("55555555555555555555555555");
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
