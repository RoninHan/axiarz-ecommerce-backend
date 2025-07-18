use std::{fs, str::FromStr};

use crate::tools::{AppState, Params, ResponseData, ResponseStatus};
use axum::{
    extract::{Multipart, Path, Query, State},
    http::StatusCode,
    response::Json,
};

use entity::{home_page_product_type, product_categories, products};
use service::{
    home_page_product_type::{HomePageProductTypeModel, HomePageProductTypeServices},
    porducts::{PorductModel, PorductServices},
    product_categories::{ProductCategoryModel, ProductCategoryServices},
    sea_orm::prelude::{DateTimeWithTimeZone, Decimal},
};

use serde_json::json;
use serde_json::to_value;

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
    pub sku: Option<String>,
    pub type_name: Option<String>,
    pub brand: Option<String>,
    pub product_details: Option<String>,
    pub product_information: Option<String>,
    pub configuration_list: Option<String>,
    pub wass: Option<String>,
    pub is_new: i32,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Debug, serde::Serialize)]
pub struct HomeProductModal {
    pub name: String,
    pub image_url: Option<String>,
    pub description: Option<String>,
    pub data: Vec<entity::products::Model>,

}

pub struct PorductController;

impl PorductController {
    pub async fn list_porducts(
        state: State<AppState>,
        Query(params): Query<Params>,
    ) -> Result<Json<serde_json::Value>, (StatusCode, &'static str)> {
        let page = params.page.unwrap_or(1);
        let posts_per_page = params.posts_per_page.unwrap_or(5);
        let q = params.q;
        let categories_id = params.categories_id;

        let (porducts, total_pages, num_pages) =
            PorductServices::get_porducts_by_page(&state.conn, page, posts_per_page, q, categories_id)
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
                sku: porduct.sku,
                type_name: porduct.type_name,
                brand: porduct.brand,
                product_details: porduct.product_details,
                product_information: porduct.product_information,
                configuration_list: porduct.configuration_list,
                wass: porduct.wass,
                is_new: porduct.is_new,
                created_at: porduct.created_at,
                updated_at: porduct.updated_at,
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
            data: Some(json!({
                "rows": porducts,
                "total_pages": total_pages,
                "num_pages": num_pages,
            })),
            code: 200,
            message: Some("Porducts retrieved successfully".to_string()),
        };
        let json_data = to_value(data).unwrap();
        //println!("Json data: {:?}", json_data);
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
        let mut sku = None;
        let mut type_name = None;
        let mut brand = None;
        let mut product_details = None;
        let mut product_information = None;
        let mut configuration_list = None;
        let mut wass = None;
        let mut is_new = None;

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
            } else if name == "image_url" {
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
            } else if name == "sku" {
                let data = field.text().await.map_err(|_| {
                    (
                        StatusCode::BAD_REQUEST,
                        "Failed to read sku field from form data",
                    )
                })?;
                sku = Some(data);
            } else if name == "type_name" {
                let data = field.text().await.map_err(|_| {
                    (
                        StatusCode::BAD_REQUEST,
                        "Failed to read type_name field from form data",
                    )
                })?;
                type_name = Some(data);
            } else if name == "brand" {
                let data = field.text().await.map_err(|_| {
                    (
                        StatusCode::BAD_REQUEST,
                        "Failed to read brand field from form data",
                    )
                })?;
                brand = Some(data);
            } else if name == "product_details" {
                let data = field.text().await.map_err(|_| {
                    (
                        StatusCode::BAD_REQUEST,
                        "Failed to read product_details field from form data",
                    )
                })?;
                product_details = Some(data);
            } else if name == "product_information" {
                let data = field.text().await.map_err(|_| {
                    (
                        StatusCode::BAD_REQUEST,
                        "Failed to read product_information field from form data",
                    )
                })?;
                product_information = Some(data);
            } else if name == "configuration_list" {
                let data = field.text().await.map_err(|_| {
                    (
                        StatusCode::BAD_REQUEST,
                        "Failed to read configuration_list field from form data",
                    )
                })?;
                configuration_list = Some(data);
            } else if name == "wass" {
                let data = field.text().await.map_err(|_| {
                    (
                        StatusCode::BAD_REQUEST,
                        "Failed to read wass field from form data",
                    )
                })?;
                wass = Some(data);
            } else if name == "is_new" {
                let data = field.text().await.map_err(|_| {
                    (
                        StatusCode::BAD_REQUEST,
                        "Failed to read is_new field from form data",
                    )
                })?;
                is_new = Some(data);
            }
        }

        let product_data = PorductModel {
            name: product_name.unwrap(),
            status: status.unwrap().parse().unwrap(),
            description: description,
            stock_quantity: stock_quantity.unwrap().parse().unwrap(),
            price: Decimal::from_str(price.as_ref().unwrap()).unwrap(),
            image_url: image_url,
            sku: sku,
            type_name: type_name,
            brand: brand,
            product_details: product_details,
            product_information: product_information,
            configuration_list: configuration_list,
            wass: wass,
            is_new: is_new.unwrap_or("1".to_string()).parse().unwrap(),
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

        let data = ResponseData::<Option<serde_json::Value>> {
            status: ResponseStatus::Success,
            data: None,
            code: 201,
            message: Some("Porduct created successfully".to_string()),
        };
        let json_data = to_value(data).unwrap();
        //println!("Json data: {:?}", json_data);
        Ok(Json(json!(json_data)))
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
        let mut sku = None;
        let mut type_name = None;
        let mut brand = None;
        let mut product_details = None;
        let mut product_information = None;
        let mut configuration_list = None;
        let mut wass = None;
        let mut is_new = None;

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
            } else if name == "image_url" {
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
            } else if name == "sku" {
                let data = field.text().await.map_err(|_| {
                    (
                        StatusCode::BAD_REQUEST,
                        "Failed to read sku field from form data",
                    )
                })?;
                sku = Some(data);
            } else if name == "type_name" {
                let data = field.text().await.map_err(|_| {
                    (
                        StatusCode::BAD_REQUEST,
                        "Failed to read type_name field from form data",
                    )
                })?;
                type_name = Some(data);
            } else if name == "brand" {
                let data = field.text().await.map_err(|_| {
                    (
                        StatusCode::BAD_REQUEST,
                        "Failed to read brand field from form data",
                    )
                })?;
                brand = Some(data);
            } else if name == "product_details" {
                let data = field.text().await.map_err(|_| {
                    (
                        StatusCode::BAD_REQUEST,
                        "Failed to read product_details field from form data",
                    )
                })?;
                product_details = Some(data);
            } else if name == "product_information" {
                let data = field.text().await.map_err(|_| {
                    (
                        StatusCode::BAD_REQUEST,
                        "Failed to read product_information field from form data",
                    )
                })?;
                product_information = Some(data);
            } else if name == "configuration_list" {
                let data = field.text().await.map_err(|_| {
                    (
                        StatusCode::BAD_REQUEST,
                        "Failed to read configuration_list field from form data",
                    )
                })?;
                configuration_list = Some(data);
            } else if name == "wass" {
                let data = field.text().await.map_err(|_| {
                    (
                        StatusCode::BAD_REQUEST,
                        "Failed to read wass field from form data",
                    )
                })?;
                println!("{}", data);
                wass = Some(data);
            } else if name == "is_new" {
                let data = field.text().await.map_err(|_| {
                    (
                        StatusCode::BAD_REQUEST,
                        "Failed to read is_new field from form data",
                    )
                })?;
                is_new = Some(data);
            }
        }
        let porduct = PorductServices::get_porduct_by_id(&state.conn, id)
            .await
            .map_err(|e| {
                println!("Failed to find porduct: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Failed to find porduct")
            })?;
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
            sku: sku.or(porduct.sku),
            type_name: type_name.or(porduct.type_name),
            brand: brand.or(porduct.brand),
            product_details: product_details.or(porduct.product_details),
            product_information: product_information.or(porduct.product_information),
            configuration_list: configuration_list.or(porduct.configuration_list),
            wass: wass.or(porduct.wass),
            is_new: is_new
                .unwrap_or(porduct.is_new.to_string())
                .parse()
                .unwrap(),
        };

        println!("Product data: {:?}", product_data);

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
        let data = ResponseData::<Option<serde_json::Value>> {
            status: ResponseStatus::Success,
            data: None,
            code: 200,
            message: Some("Porduct updated successfully".to_string()),
        };
        let json_data = to_value(data).unwrap();
        //println!("Json data: {:?}", json_data);
        Ok(Json(json!(json_data)))
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

        let data = ResponseData::<Option<serde_json::Value>> {
            status: ResponseStatus::Success,
            data: None,
            code: 200,
            message: Some("Porduct deleted successfully".to_string()),
        };
        let json_data = to_value(data).unwrap();
        //println!("Json data: {:?}", json_data);
        Ok(Json(json!(json_data)))
    }

    pub async fn get_product_by_home_product_type(
        state: State<AppState>,
    ) -> Result<Json<serde_json::Value>, (StatusCode, &'static str)> {
        let home_page_product_type =
            HomePageProductTypeServices::get_home_page_product_type_all(&state.conn)
                .await
                .map_err(|e| {
                    println!("Failed to get porduct by home product type: {:?}", e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Failed to get porduct by home product type",
                    )
                })?;

        let mut porducts: Vec<HomeProductModal> = vec![];
        for home_page_product_type in home_page_product_type {
            let product_category = ProductCategoryServices::find_by_category_id(
                &state.conn,
                home_page_product_type.product_type_id,
            )
            .await
            .map_err(|e| {
                println!("Failed to get product category: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to get product category",
                )
            })?;

            let mut porducts_temp: Vec<entity::products::Model> = vec![];
            for product_category in product_category {
                let porduct: entity::products::Model =
                    PorductServices::get_porduct_by_id(&state.conn, product_category.product_id)
                        .await
                        .map_err(|e| {
                            println!("Failed to get porduct by id: {:?}", e);
                            (
                                StatusCode::INTERNAL_SERVER_ERROR,
                                "Failed to get porduct by id",
                            )
                        })?;

                porducts_temp.push(porduct);
            }

            let home_product: HomeProductModal = HomeProductModal {
                name: home_page_product_type.name,
                image_url: Some(home_page_product_type.image_url),
                description: home_page_product_type.description,
                data: porducts_temp,
            };
            porducts.push(home_product);
        }

        let data = ResponseData {
            status: ResponseStatus::Success,
            data: Some(json!(porducts)),
            code: 200,
            message: Some("Porducts retrieved successfully".to_string()),
        };

        let json_data = to_value(data).unwrap();
        Ok(Json(json!(json_data)))
    }

    pub async fn create_home_product(
        state: State<AppState>,
        mut multipart: Multipart,
    ) -> Result<Json<serde_json::Value>, (StatusCode, &'static str)> {
        let mut product_type_id = None;
        let mut home_product_name = None;
        let mut description = None;
        let mut image_url = None;

        while let Some(field) = multipart.next_field().await.map_err(|_| {
            (
                StatusCode::BAD_REQUEST,
                "Failed to process multipart form data",
            )
        })? {
            let name = field.name().unwrap_or("").to_string();
            if name == "product_type_id" {
                let data = field.text().await.map_err(|_| {
                    (
                        StatusCode::BAD_REQUEST,
                        "Failed to read product_type_id field from form data",
                    )
                })?;
                product_type_id = Some(data);
            } else if name == "name" {
                let data = field.text().await.map_err(|_| {
                    (
                        StatusCode::BAD_REQUEST,
                        "Failed to read name field from form data",
                    )
                })?;
                home_product_name = Some(data);
            } else if name == "description" {
                let data = field.text().await.map_err(|_| {
                    (
                        StatusCode::BAD_REQUEST,
                        "Failed to read description field from form data",
                    )
                })?;
                description = Some(data);
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
            }
        }

        let home_page_product_type_data = HomePageProductTypeModel {
            product_type_id: product_type_id.unwrap().parse().unwrap(),
            name: home_product_name.unwrap(),
            description: description,
            image_url: image_url.unwrap(),
        };

        let _ = HomePageProductTypeServices::create_home_page_product_type(
            &state.conn,
            home_page_product_type_data,
        )
        .await;

        let data = ResponseData::<Option<serde_json::Value>> {
            status: ResponseStatus::Success,
            data: None,
            code: 201,
            message: Some("Home page product type created successfully".to_string()),
        };
        let json_data = to_value(data).unwrap();
        //println!("Json data: {:?}", json_data);
        Ok(Json(json!(json_data)))
    }

    pub async fn delete_home_product(
        state: State<AppState>,
        Path(id): Path<i32>,
    ) -> Result<Json<serde_json::Value>, (StatusCode, &'static str)> {
        HomePageProductTypeServices::delete_home_page_product_type_by_id(&state.conn, id)
            .await
            .map_err(|e| {
                println!("Failed to delete home page product type: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to delete home page product type",
                )
            })?;

        let data = ResponseData::<Option<serde_json::Value>> {
            status: ResponseStatus::Success,
            data: None,
            code: 200,
            message: Some("Home page product type deleted successfully".to_string()),
        };
        let json_data = to_value(data).unwrap();
        //println!("Json data: {:?}", json_data);
        Ok(Json(json!(json_data)))
    }

    pub async fn get_product_by_id(
        state: State<AppState>,
        Path(id): Path<i32>,
    ) -> Result<Json<serde_json::Value>, (StatusCode, &'static str)> {
        let porduct = PorductServices::get_porduct_by_id(&state.conn, id)
            .await
            .map_err(|e| {
                println!("Failed to get porduct by id: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Failed to get porduct by id")
            })?;

        let data = ResponseData {
            status: ResponseStatus::Success,
            data: Some(json!(porduct)),
            code: 200,
            message: Some("Product retrieved successfully".to_string()),
        };
        let json_data = to_value(data).unwrap();
        //println!("Json data: {:?}", json_data);
        Ok(Json(json!(json_data)))
    }

    pub async fn get_product_by_new(
        state: State<AppState>,
    ) -> Result<Json<serde_json::Value>, (StatusCode, &'static str)> {
        let porduct = PorductServices::get_product_by_new(&state.conn)
            .await
            .map_err(|e| {
                println!("Failed to get porduct by new: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to get porduct by new",
                )
            })?;

        let data = ResponseData {
            status: ResponseStatus::Success,
            message: Some("Product retrieved successfully".to_string()),
            data: Some(json!(porduct)),
            code: 200,
        };
        let json_data = to_value(data).unwrap();
        //println!("Json data: {:?}", json_data);
        Ok(Json(json!(json_data)))
    }
}
