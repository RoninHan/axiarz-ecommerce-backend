use ::entity::{reviews, reviews::Entity as Review};
use chrono::{DateTime, Utc};
use prelude::DateTimeWithTimeZone;
use sea_orm::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct ReviewModel {
    pub product_id: i32,
    pub user_id: i32,
    pub rating: i32,
    pub comment: Option<String>,
}

pub struct ReviewServices;

impl ReviewServices {
    /// 异步创建一条新的评论记录到数据库中。
    ///
    /// # 参数
    /// * `db` - 数据库连接的引用。
    /// * `form_data` - 包含评论数据的 `ReviewModel`。
    ///
    /// # 返回值
    /// * `Result<reviews::ActiveModel, DbErr>` - 成功时返回新创建的评论记录（`ActiveModel`），
    ///   失败时返回数据库错误（`DbErr`）。
    pub async fn create_review(
        db: &DbConn,
        form_data: ReviewModel,
    ) -> Result<reviews::ActiveModel, DbErr> {
        // 创建一个新的评论 ActiveModel，并设置各字段的值
        reviews::ActiveModel {
            product_id: Set(form_data.product_id), // 设置产品 ID
            user_id: Set(form_data.user_id),       // 设置用户 ID
            rating: Set(form_data.rating),         // 设置评分
            comment: Set(form_data.comment),       // 设置评论内容
            created_at: Set(Some(DateTimeWithTimeZone::from(Utc::now()))), // 设置创建时间为当前时间
            ..Default::default()                   // 其他字段使用默认值
        }
        .save(db) // 保存 ActiveModel 到数据库
        .await
    }

    /// 根据评论 ID 更新评论记录。
    ///
    /// # 参数
    /// * `db` - 数据库连接的引用。
    /// * `id` - 要更新的评论记录的 ID。
    /// * `form_data` - 包含更新数据的 `ReviewModel`。
    ///
    /// # 返回值
    /// * `Result<reviews::Model, DbErr>` - 成功时返回更新后的评论记录，
    ///   失败时返回数据库错误（`DbErr`）。
    pub async fn update_review_by_id(
        db: &DbConn,
        id: i32,
        form_data: ReviewModel,
    ) -> Result<reviews::Model, DbErr> {
        // 查找指定 ID 的评论记录，如果不存在则返回错误
        let reviews: reviews::ActiveModel = Review::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find reviews.".to_owned())) // 如果找不到记录，返回自定义错误
            .map(Into::into)?; // 将结果转换为 ActiveModel

        // 创建一个新的 ActiveModel，更新指定字段
        reviews::ActiveModel {
            id: reviews.id,                        // 保留原始 ID
            product_id: Set(form_data.product_id), // 更新产品 ID
            user_id: Set(form_data.user_id),       // 更新用户 ID
            rating: Set(form_data.rating),         // 更新评分
            comment: Set(form_data.comment),       // 更新评论内容
            ..Default::default()                   // 其他字段使用默认值
        }
        .update(db) // 更新记录到数据库
        .await
    }

    /// 根据评论 ID 删除评论记录。
    ///
    /// # 参数
    /// * `db` - 数据库连接的引用。
    /// * `id` - 要删除的评论记录的 ID。
    ///
    /// # 返回值
    /// * `Result<DeleteResult, DbErr>` - 成功时返回删除结果，
    ///   失败时返回数据库错误（`DbErr`）。
    pub async fn delete_review_by_id(db: &DbConn, id: i32) -> Result<DeleteResult, DbErr> {
        // 查找指定 ID 的评论记录，如果不存在则返回错误
        let reviews: reviews::ActiveModel = Review::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find reviews.".to_owned())) // 如果找不到记录，返回自定义错误
            .map(Into::into)?; // 将结果转换为 ActiveModel

        // 删除找到的评论记录
        reviews.delete(db).await
    }

    /// 根据产品 ID 获取所有相关的评论。
    ///
    /// # 参数
    /// * `db` - 数据库连接的引用。
    /// * `product_id` - 产品的 ID。
    ///
    /// # 返回值
    /// * `Result<Vec<reviews::Model>, DbErr>` - 成功时返回评论列表，
    ///   失败时返回数据库错误（`DbErr`）。
    pub async fn get_reviews_by_product_id(
        db: &DbConn,
        product_id: i32,
    ) -> Result<Vec<reviews::Model>, DbErr> {
        Review::find()
            .filter(reviews::Column::ProductId.eq(product_id)) // 过滤条件：匹配产品 ID
            .all(db) // 查询所有匹配的记录
            .await
    }

    /// 根据用户 ID 获取所有相关的评论。
    ///
    /// # 参数
    /// * `db` - 数据库连接的引用。
    /// * `user_id` - 用户的 ID。
    ///
    /// # 返回值
    /// * `Result<Vec<reviews::Model>, DbErr>` - 成功时返回评论列表，
    ///   失败时返回数据库错误（`DbErr`）。
    pub async fn get_reviews_by_user_id(
        db: &DbConn,
        user_id: i32,
    ) -> Result<Vec<reviews::Model>, DbErr> {
        Review::find()
            .filter(reviews::Column::UserId.eq(user_id)) // 过滤条件：匹配用户 ID
            .all(db) // 查询所有匹配的记录
            .await
    }

    /// 根据评论 ID 获取单条评论。
    ///
    /// # 参数
    /// * `db` - 数据库连接的引用。
    /// * `id` - 评论的 ID。
    ///
    /// # 返回值
    /// * `Result<reviews::Model, DbErr>` - 成功时返回评论记录，
    ///   失败时返回数据库错误（`DbErr`）。
    pub async fn get_review_by_id(db: &DbConn, id: i32) -> Result<reviews::Model, DbErr> {
        Review::find_by_id(id) // 根据 ID 查找评论
            .one(db) // 查询单条记录
            .await?
            .ok_or(DbErr::Custom("Cannot find review.".to_owned())) // 如果未找到，返回自定义错误
    }

    /// 获取所有评论。
    ///
    /// # 参数
    /// * `db` - 数据库连接的引用。
    ///
    /// # 返回值
    /// * `Result<Vec<reviews::Model>, DbErr>` - 成功时返回所有评论列表，
    ///   失败时返回数据库错误（`DbErr`）。
    pub async fn get_reviews_all(db: &DbConn) -> Result<Vec<reviews::Model>, DbErr> {
        Review::find() // 查找所有评论
            .all(db) // 查询所有记录
            .await
    }

    // 分頁
    pub async fn get_reviews(
        db: &DbConn,
        page: u64,
        limit: u64,
    ) -> Result<(Vec<reviews::Model>, u64), DbErr> {
        let paginator = Review::find()
            .order_by_asc(reviews::Column::Id)
            .paginate(db, limit);
        let num_pages = paginator.num_pages().await?;

        paginator.fetch_page(page - 1).await.map(|p| (p, num_pages))
    }
}
