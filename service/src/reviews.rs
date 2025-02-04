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
    pub async fn create_review(
        db: &DbConn,
        form_data: ReviewModel,
    ) -> Result<reviews::ActiveModel, DbErr> {
        reviews::ActiveModel {
            product_id: Set(form_data.product_id),
            user_id: Set(form_data.user_id),
            rating: Set(form_data.rating),
            comment: Set(form_data.comment),
            created_at: Set(Some(DateTimeWithTimeZone::from(Utc::now()))),
            ..Default::default()
        }
        .save(db)
        .await
    }

    pub async fn update_review_by_id(
        db: &DbConn,
        id: i32,
        form_data: ReviewModel,
    ) -> Result<reviews::Model, DbErr> {
        let reviews: reviews::ActiveModel = Review::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find reviews.".to_owned()))
            .map(Into::into)?;
        reviews::ActiveModel {
            id: reviews.id,
            product_id: Set(form_data.product_id),
            user_id: Set(form_data.user_id),
            rating: Set(form_data.rating),
            comment: Set(form_data.comment),
            ..Default::default()
        }
        .update(db)
        .await
    }

    pub async fn delete_review_by_id(db: &DbConn, id: i32) -> Result<DeleteResult, DbErr> {
        let reviews: reviews::ActiveModel = Review::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find reviews.".to_owned()))
            .map(Into::into)?;

        reviews.delete(db).await
    }

    pub async fn get_reviews_by_product_id(
        db: &DbConn,
        product_id: i32,
    ) -> Result<Vec<reviews::Model>, DbErr> {
        Review::find()
            .filter(reviews::Column::ProductId.eq(product_id))
            .all(db)
            .await
    }

    pub async fn get_reviews_by_user_id(
        db: &DbConn,
        user_id: i32,
    ) -> Result<Vec<reviews::Model>, DbErr> {
        Review::find()
            .filter(reviews::Column::UserId.eq(user_id))
            .all(db)
            .await
    }

    pub async fn get_review_by_id(db: &DbConn, id: i32) -> Result<reviews::Model, DbErr> {
        Review::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find review.".to_owned()))
    }

    pub async fn get_reviews_all(db: &DbConn) -> Result<Vec<reviews::Model>, DbErr> {
        Review::find().all(db).await
    }

    pub async fn get_reviews_by_rating(
        db: &DbConn,
        rating: i32,
    ) -> Result<Vec<reviews::Model>, DbErr> {
        Review::find()
            .filter(reviews::Column::Rating.eq(rating))
            .all(db)
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
