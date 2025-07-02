use std::fs;

use crate::tools::{AppState, Params, ResponseData, ResponseStatus};
use axum::{
    extract::{Multipart, Path, Query, State},
    http::StatusCode,
    response::Json,
};
use serde_json::json;
use serde_json::to_value;
use service::banner::{BannerModel, BannerServices};

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
                (StatusCode::INTERNAL_SERVER_ERROR, "Failed to get banners")
            })?;

        let data = ResponseData {
            status: ResponseStatus::Success,
            data: { json!(banners) },
        };

        let json_data = to_value(data).unwrap();
        println!("Json data: {:?}", json_data);

        Ok(Json(json!(json_data)))
    }

    pub async fn create_banner(
        state: State<AppState>,
        mut multipart: Multipart,
    ) -> Result<Json<serde_json::Value>, (StatusCode, &'static str)> {
        let mut title = None;
        let mut link = None;
        let mut image_path: Option<String> = None;

        while let Some(field) = multipart.next_field().await.map_err(|_| {
            (
                StatusCode::BAD_REQUEST,
                "Failed to process multipart form data",
            )
        })? {
            let name = field.name().unwrap_or("").to_string();
            if name == "title" {
                // 提取標題
                let data = field.text().await.map_err(|_| {
                    (
                        StatusCode::BAD_REQUEST,
                        "Failed to read title field from form data",
                    )
                })?;
                title = Some(data);
            } else if name == "link" {
                // 提取鏈接
                let data = field.text().await.map_err(|_| {
                    (
                        StatusCode::BAD_REQUEST,
                        "Failed to read link field from form data",
                    )
                })?;
                link = Some(data);
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
                image_path = Some(file_path.replace("./", "/"));
                println!("Image path: {:?}", image_path);
            }
        }

        let banner_data = BannerModel {
            title: title.ok_or((StatusCode::BAD_REQUEST, "Missing title"))?,
            link: link.ok_or((StatusCode::BAD_REQUEST, "Missing link"))?,
            image_url: image_path
                .clone()
                .ok_or((StatusCode::BAD_REQUEST, "Missing image file"))?,
        };

        // 調用服務層創建 Banner
        BannerServices::create_banner(&state.conn, banner_data)
            .await
            .map_err(|e| {
                println!("Failed to create banner: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create banner")
            })?;

        Ok(Json(json!({
            "status": "success",
            "message": "Banner created successfully"
        })))
    }

    pub async fn update_banner(
        state: State<AppState>,
        Path(id): Path<i32>,
        mut multipart: Multipart,
    ) -> Result<Json<serde_json::Value>, (StatusCode, &'static str)> {
        let mut title: Option<String> = None;
        let mut link: Option<String> = None;
        let mut image_path: Option<String> = None;

        while let Some(field) = multipart.next_field().await.map_err(|_| {
            (
                StatusCode::BAD_REQUEST,
                "Failed to process multipart form data",
            )
        })? {
            let name = field.name().unwrap_or("").to_string();
            if name == "title" {
                // 提取标题字段
                title = Some(
                    field
                        .text()
                        .await
                        .map_err(|_| (StatusCode::BAD_REQUEST, "Failed to read title field"))?,
                );
            } else if name == "link" {
                // 提取链接字段
                link = Some(
                    field
                        .text()
                        .await
                        .map_err(|_| (StatusCode::BAD_REQUEST, "Failed to read link field"))?,
                );
            } else if name == "image" {
                // 提取图片文件
                let file_name = field.file_name().unwrap_or("default.png").to_string();
                let file_data = field
                    .bytes()
                    .await
                    .map_err(|_| (StatusCode::BAD_REQUEST, "Failed to read image file"))?;

                // 确保目标目录存在
                let upload_dir = "./uploads";
                if !std::path::Path::new(upload_dir).exists() {
                    std::fs::create_dir_all(upload_dir).map_err(|e| {
                        eprintln!("Failed to create upload directory: {:?}", e);
                        (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            "Failed to create upload directory",
                        )
                    })?;
                }

                // 保存图片到服务器
                let file_path = format!("{}/{}", upload_dir, file_name);
                fs::write(&file_path, &file_data).map_err(|e| {
                    eprintln!("Failed to save image file: {:?}", e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Failed to save image file",
                    )
                })?;
                image_path = Some(file_path.replace("./", "/"));
            }
        }

        // 确保至少有一个字段被更新
        if title.is_none() && link.is_none() && image_path.is_none() {
            return Err((
                StatusCode::BAD_REQUEST,
                "At least one field (title, link, or image) must be provided",
            ));
        }

        let banner = BannerServices::get_banner_by_id(&state.conn, id)
            .await
            .map_err(|e| {
                println!("Failed to get banner: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Failed to get banner")
            })?;

        if title.is_none() {
            title = Some(banner.title);
        }
        if link.is_none() {
            link = Some(banner.link);
        }
        if image_path.is_none() {
            image_path = Some(banner.image_url);
        }

        let payload = BannerModel {
            title: title.unwrap_or_default(),
            link: link.unwrap_or_default(),
            image_url: image_path.unwrap_or_default(),
        };

        println!("Payload: {:?}", payload);
        BannerServices::update_banner_by_id(&state.conn, id, payload)
            .await
            .map_err(|e| {
                println!("Failed to update banner: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Failed to update banner")
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
                (StatusCode::INTERNAL_SERVER_ERROR, "Failed to delete banner")
            })?;

        Ok(Json(json!({
            "status": "success",
            "message": "Banner deleted successfully"
        })))
    }
}
