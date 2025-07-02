use crate::{
    middleware::auth::Auth,
    tools::{AppState, Params, ResponseData, ResponseStatus},
};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
};
use service::user::{LoginModel, UserModel, UserServices};

use serde_json::json;
use serde_json::to_value;

pub struct UserController;

impl UserController {
    pub async fn list_users(
        state: State<AppState>,
        Query(params): Query<Params>,
    ) -> Result<Json<serde_json::Value>, (StatusCode, &'static str)> {
        let page = params.page.unwrap_or(1);
        let posts_per_page = params.posts_per_page.unwrap_or(5);

        let (users, num_pages) = UserServices::find_user(&state.conn, page, posts_per_page)
            .await
            .expect("Cannot find posts in page");

        let data = ResponseData {
            code: 200,
            status: ResponseStatus::Success,
            data: Some(json!({
                "rows": users,
                "num_pages": num_pages,
            })),
            message: Some("Users retrieved successfully".to_string()),
        };

        let json_data = to_value(data).unwrap();
        println!("Json data: {:?}", json_data);
        Ok(Json(json!(json_data)))
    }

    pub async fn create_user(
        state: State<AppState>,
        Json(payload): Json<UserModel>,
    ) -> Result<Json<serde_json::Value>, (StatusCode, &'static str)> {
        println!("Payload: {:?}", payload);
        // password md5
        let payload = UserModel {
            password: Auth::hash_password(&payload.password)
                .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to hash password"))?,
            ..payload
        };
        UserServices::create_user(&state.conn, payload)
            .await
            .map_err(|e| {
                println!("Failed to create user: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create user")
            })?;

        let data = ResponseData::<Option<serde_json::Value>> {
            code: 201,
            status: ResponseStatus::Success,
            data: None,
            message: Some("User created successfully".to_string()),
        };
        let json_data = to_value(data).unwrap();
        println!("Json data: {:?}", json_data);
        Ok(Json(json!(json_data)))
    }

    pub async fn update_user(
        state: State<AppState>,
        Path(id): Path<i32>,
        Json(payload): Json<UserModel>,
    ) -> Result<Json<serde_json::Value>, (StatusCode, &'static str)> {
        println!("Payload: {:?}", payload);
        UserServices::update_user_by_id(&state.conn, id, payload)
            .await
            .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to update user"))?;

        let data = ResponseData::<Option<serde_json::Value>> {
            code: 200,
            status: ResponseStatus::Success,
            data: None,
            message: Some("User updated successfully".to_string()),
        };
        let json_data = to_value(data).unwrap();
        println!("Json data: {:?}", json_data);
        Ok(Json(json!(json_data)))
    }

    pub async fn delete_user(
        state: State<AppState>,
        Path(id): Path<i32>,
    ) -> Result<Json<serde_json::Value>, (StatusCode, &'static str)> {
        UserServices::delete_user(&state.conn, id)
            .await
            .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to delete user"))?;

        let data = ResponseData::<Option<serde_json::Value>> {
            code: 200,
            status: ResponseStatus::Success,
            data: None,
            message: Some("User deleted successfully".to_string()),
        };
        let json_data = to_value(data).unwrap();
        println!("Json data: {:?}", json_data);
        Ok(Json(json!(json_data)))  
    }

    pub async fn get_user_by_id(
        state: State<AppState>,
        Path(id): Path<i32>,
    ) -> Result<Json<serde_json::Value>, (StatusCode, &'static str)> {
        let user = UserServices::find_user_by_id(&state.conn, id)
            .await
            .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to find user"))?;

        let data = match user {
            Some(user) => ResponseData {
                code: 200,
                status: ResponseStatus::Success,
                data: Some(json!(user)),
                message: Some("User retrieved successfully".to_string()),
            },
            None => ResponseData {
                code: 404,
                status: ResponseStatus::Error,
                data: None,
                message: Some("User not found".to_string()),
            },
        };
        let json_data = to_value(data).unwrap();
        println!("Json data: {:?}", json_data);
        Ok(Json(json!(json_data)))
    }

    pub async fn login(
        state: State<AppState>,
        Json(payload): Json<LoginModel>,
    ) -> Result<Json<serde_json::Value>, (StatusCode, &'static str)> {
        let email = &payload.email;
        let password = &payload.password;
        // Check if email and password are empty
        if email.is_empty() || password.is_empty() {
            return Err((StatusCode::BAD_REQUEST, "Email and password are required"));
        }

        // Find user by email
        let user = UserServices::find_user_by_email(&state.conn, email)
            .await
            .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to find user"))?;

        // Check if user is found
        let mut user = user.unwrap();

        // Check if password is found
        let hashed_password = &user.password;

        // Verify password
        match Auth::verify_password(password, hashed_password) {
            Ok(is_valid) => {
                if is_valid {
                    // Generate JWT token
                    let token = Auth::encode_jwt(email.to_string()).map_err(|_| {
                        (StatusCode::INTERNAL_SERVER_ERROR, "Failed to encode token")
                    })?;

                    // Convert Model to UserModel (implement From<Model> for UserModel if not already)
                    
                    user.password = "".to_string();

                    let data = ResponseData {
                        code: 200,
                        status: ResponseStatus::Success,
                        data: Some(json!({
                            "token": token,
                            "user": user,
                        })),
                        message: Some("Login successful".to_string()),
                    };
                    let json_data = to_value(data).unwrap();
                    Ok(Json(json!(json_data)))
                } else {
                    Err((StatusCode::UNAUTHORIZED, "Invalid password"))
                }
            }
            Err(_) => Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to verify password",
            )),
        }
    }

    pub async fn register(
        state: State<AppState>,
        Json(payload): Json<UserModel>,
    ) -> Result<Json<serde_json::Value>, (StatusCode, &'static str)> {
        let payload = UserModel {
            password: Auth::hash_password(&payload.password)
                .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to hash password"))?,
            ..payload
        };
        UserServices::create_user(&state.conn, payload)
            .await
            .map_err(|e| {
                println!("Failed to register user: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Failed to register user")
            })?;

       let data = ResponseData::<Option<serde_json::Value>> {
            code: 201,
            status: ResponseStatus::Success,
            data: None,
            message: Some("User registered successfully".to_string()),
        };
        let json_data = to_value(data).unwrap();
        println!("Json data: {:?}", json_data);
        Ok(Json(json!(json_data)))
    }
}
