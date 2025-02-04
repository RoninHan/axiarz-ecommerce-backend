use ::entity::{product_categories, product_categories::Entity as ProductCategory};
use chrono::{DateTime, Utc};
use prelude::DateTimeWithTimeZone;
use sea_orm::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct ProductCategoryModel {
    pub product_id: i32,
    pub category_id: i32,
}

pub struct ProductCategoryServices;

impl ProductCategoryServices {
    pub async fn create_product_category(
        db: &DbConn,
        form_data: ProductCategoryModel,
    ) -> Result<product_categories::ActiveModel, DbErr> {
        product_categories::ActiveModel {
            product_id: Set(form_data.product_id),
            category_id: Set(form_data.category_id),
            ..Default::default()
        }
        .save(db)
        .await
    }

    pub async fn delete_product_category_by_product_id(
        db: &DbConn,
        product_id: i32,
    ) -> Result<(), DbErr> {
        let product_categories = ProductCategory::find()
            .filter(product_categories::Column::ProductId.eq(product_id))
            .all(db)
            .await?;

        for product_category in product_categories {
            product_category.delete(db).await?;
        }

        Ok(())
    }

    pub async fn delete_product_category_by_category_id(
        db: &DbConn,
        category_id: i32,
    ) -> Result<(), DbErr> {
        let product_categories = ProductCategory::find()
            .filter(product_categories::Column::CategoryId.eq(category_id))
            .all(db)
            .await?;

        for product_category in product_categories {
            product_category.delete(db).await?;
        }

        Ok(())
    }

    pub async fn get_all_product_categories(
        db: &DbConn,
    ) -> Result<Vec<product_categories::Model>, DbErr> {
        ProductCategory::find().all(db).await
    }
}
