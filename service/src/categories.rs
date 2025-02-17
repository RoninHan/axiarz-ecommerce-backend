use ::entity::{categories, categories::Entity as Category};
use chrono::{DateTime, Utc};
use prelude::DateTimeWithTimeZone;
use sea_orm::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct CategoryModel {
    pub name: String,
    pub description: Option<String>,
    pub parent_id: Option<i32>,
}

pub struct CategoryServices;

impl CategoryServices {
    pub async fn create_category(
        db: &DbConn,
        form_data: CategoryModel,
    ) -> Result<categories::ActiveModel, DbErr> {
        categories::ActiveModel {
            name: Set(form_data.name.to_owned()),
            description: Set(form_data.description.to_owned()),
            parent_id: Set(form_data.parent_id),
            created_at: Set(Some(DateTimeWithTimeZone::from(Utc::now()))),
            updated_at: Set(Some(DateTimeWithTimeZone::from(Utc::now()))),
            ..Default::default()
        }
        .save(db)
        .await
    }

    pub async fn update_category_by_id(
        db: &DbConn,
        id: i32,
        form_data: CategoryModel,
    ) -> Result<categories::Model, DbErr> {
        let categories: categories::ActiveModel = Category::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find categories.".to_owned()))
            .map(Into::into)?;
        categories::ActiveModel {
            id: categories.id,
            name: Set(form_data.name.to_owned()),
            description: Set(form_data.description.to_owned()),
            parent_id: Set(form_data.parent_id),
            ..Default::default()
        }
        .update(db)
        .await
    }

    pub async fn delete_category_by_id(db: &DbConn, id: i32) -> Result<DeleteResult, DbErr> {
        let categories: categories::ActiveModel = Category::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find categories.".to_owned()))
            .map(Into::into)?;

        categories.delete(db).await
    }

    pub async fn get_categories(db: &DbConn) -> Result<Vec<categories::Model>, DbErr> {
        Category::find().all(db).await
    }

    pub async fn get_categories_by_page(
        db: &DbConn,
        page: u64,
        size: u64,
    ) -> Result<(Vec<categories::Model>, u64), DbErr> {
        let paginator = Category::find()
            .order_by_asc(categories::Column::Id)
            .paginate(db, size);
        let num_pages = paginator.num_pages().await?;

        paginator.fetch_page(page - 1).await.map(|p| (p, num_pages))
    }
}
