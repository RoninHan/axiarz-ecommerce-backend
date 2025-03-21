use ::entity::{cart_items, cart_items::Entity as CartItem};
use chrono::{DateTime, Utc};
use prelude::DateTimeWithTimeZone;
use sea_orm::*;
use serde::{Deserialize, Serialize};

// 定义一个数据模型，用于表示购物车项的结构
#[derive(Deserialize, Serialize, Debug)]
pub struct CartItemModel {
    pub user_id: i32,    // 用户ID
    pub product_id: i32, // 产品ID
    pub quantity: i32,   // 产品数量
}

// 定义一个服务结构体，用于封装购物车项的相关操作
pub struct CartItemServices;

impl CartItemServices {
    // 创建购物车项
    pub async fn create_cart_item(
        db: &DbConn,              // 数据库连接
        form_data: CartItemModel, // 传入的购物车项数据
    ) -> Result<cart_items::ActiveModel, DbErr> {
        // 构建一个新的购物车项并保存到数据库
        cart_items::ActiveModel {
            user_id: Set(form_data.user_id),       // 设置用户ID
            product_id: Set(form_data.product_id), // 设置产品ID
            quantity: Set(form_data.quantity),     // 设置产品数量
            added_at: Set(Some(DateTimeWithTimeZone::from(Utc::now()))), // 设置添加时间
            ..Default::default()                   // 其他字段使用默认值
        }
        .save(db) // 保存到数据库
        .await // 异步等待保存结果
    }

    // 根据ID更新购物车项
    pub async fn update_cart_item_by_id(
        db: &DbConn,              // 数据库连接
        id: i32,                  // 要更新的购物车项ID
        form_data: CartItemModel, // 更新的数据
    ) -> Result<cart_items::Model, DbErr> {
        // 查找指定ID的购物车项
        let cart_items: cart_items::ActiveModel = CartItem::find_by_id(id)
            .one(db) // 查询数据库
            .await? // 异步等待查询结果
            .ok_or(DbErr::Custom("Cannot find cart_items.".to_owned())) // 如果找不到，返回错误
            .map(Into::into)?; // 将结果转换为ActiveModel

        // 更新购物车项的字段
        cart_items::ActiveModel {
            id: cart_items.id,                     // 保留原ID
            user_id: Set(form_data.user_id),       // 更新用户ID
            product_id: Set(form_data.product_id), // 更新产品ID
            quantity: Set(form_data.quantity),     // 更新产品数量
            ..Default::default()                   // 其他字段使用默认值
        }
        .update(db) // 更新到数据库
        .await // 异步等待更新结果
    }

    // 根据ID删除购物车项
    pub async fn delete_cart_item_by_id(
        db: &DbConn, // 数据库连接
        id: i32,     // 要删除的购物车项ID
    ) -> Result<DeleteResult, DbErr> {
        // 查找指定ID的购物车项
        let cart_items: cart_items::ActiveModel = CartItem::find_by_id(id)
            .one(db) // 查询数据库
            .await? // 异步等待查询结果
            .ok_or(DbErr::Custom("Cannot find cart_items.".to_owned())) // 如果找不到，返回错误
            .map(Into::into)?; // 将结果转换为ActiveModel

        // 删除购物车项
        cart_items.delete(db).await // 从数据库中删除
    }

    // 根据用户ID获取购物车项列表
    pub async fn get_cart_items_by_user_id(
        db: &DbConn,  // 数据库连接
        user_id: i32, // 用户ID
    ) -> Result<Vec<cart_items::Model>, DbErr> {
        // 查询指定用户ID的所有购物车项
        CartItem::find()
            .filter(cart_items::Column::UserId.eq(user_id)) // 过滤条件：用户ID匹配
            .all(db) // 查询所有匹配的记录
            .await // 异步等待查询结果
    }
}
