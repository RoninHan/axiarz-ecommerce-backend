use ::Entity::{products, products::Entity as Product};
use chrono::{DateTime, Utc};
use prelude::DateTimeWithTimeZone;
use sea_orm::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct ProductModel {
    pub name: String,
    pub price: i32,
    pub description: String,
    pub image: String,
    pub category: String,
}

pub struct ProductServices;

impl ProductServices {
    pub async fn create_product(
        db: &DbConn,
        form_data: ProductModel,
    ) -> Result<products::ActiveModel, DbErr> {
        products::ActiveModel {
            name: Set(form_data.name.to_owned()),
            price: Set(form_data.price),
            description: Set(form_data.description.to_owned()),
            image: Set(form_data.image.to_owned()),
            category: Set(form_data.category.to_owned()),
            created_at: Set(DateTimeWithTimeZone::from(Utc::now())),
            updated_at: Set(DateTimeWithTimeZone::from(Utc::now())),
            ..Default::default()
        }
        .save(db)
        .await
    }

    pub async fn update_product_by_id(
        db: &DbConn,
        id: i32,
        form_data: ProductModel,
    ) -> Result<products::Model, DbErr> {
        let products: products::ActiveModel = Product::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find products.".to_owned()))
            .map(Into::into)?;
        products::ActiveModel {
            id: products.id,
            name: Set(form_data.name.to_owned()),
            price: Set(form_data.price),
            description: Set(form_data.description.to_owned()),
            image: Set(form_data.image.to_owned()),
            category: Set(form_data.category.to_owned()),
            ..products
        }
        .save(db)
        .await
    }

    pub async fn delete_product_by_id(db: &DbConn, id: i32) -> Result<(), DbErr> {
        Product::delete()
            .filter(products::Column::Id.eq(id))
            .exec(db)
            .await?;
        Ok(())
    }

    pub async fn get_all_products(db: &DbConn) -> Result<Vec<products::Model>, DbErr> {
        let products: Vec<products::Model> = Product::find().all(db).await?;
        Ok(products)
    }

    pub async fn get_product_by_id(db: &DbConn, id: i32) -> Result<products::Model, DbErr> {
        let product: products::Model = Product::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find product.".to_owned()))?;
        Ok(product)
    }

    // 查詢分頁，根據name查詢
    pub async fn get_products_by_name(
        db: &DbConn,
        name: String,
        page: i64,
        limit: i64,
    ) -> Result<Vec<products::Model>, DbErr> {
        let products: Vec<products::Model> = Product::find()
            .filter(products::Column::Name.contains(name))
            .paginate(page, limit)
            .all(db)
            .await?;
        Ok(products)
    }
}
