use ::entity::{products, products::Entity as Product};
use chrono::{DateTime, Utc};
use prelude::DateTimeWithTimeZone;
use sea_orm::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct PorductModel {
    pub name: String,
    pub status: i32,
    pub description: Option<String>,
    pub stock_quantity: i32,
    pub price: prelude::Decimal,
    pub image_url: Option<String>,
}

pub struct PorductServices;

impl PorductServices {
    pub async fn create_porduct(
        db: &DbConn,
        form_data: PorductModel,
    ) -> Result<products::ActiveModel, DbErr> {
        products::ActiveModel {
            name: Set(form_data.name.to_owned()),
            status: Set(form_data.status),
            description: Set(form_data.description.to_owned()),
            stock_quantity: Set(form_data.stock_quantity),
            price: Set(form_data.price),
            image_url: Set(form_data.image_url.to_owned()),
            created_at: Set(DateTimeWithTimeZone::from(Utc::now())),
            updated_at: Set(DateTimeWithTimeZone::from(Utc::now())),
            ..Default::default()
        }
        .save(db)
        .await
    }

    pub async fn update_porduct_by_id(
        db: &DbConn,
        id: i32,
        form_data: PorductModel,
    ) -> Result<products::Model, DbErr> {
        let products: products::ActiveModel = Product::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find products.".to_owned()))
            .map(Into::into)?;
        products::ActiveModel {
            id: products.id,
            name: Set(form_data.name.to_owned()),
            status: Set(form_data.status),
            description: Set(form_data.description.to_owned()),
            stock_quantity: Set(form_data.stock_quantity),
            price: Set(form_data.price),
            image_url: Set(form_data.image_url.to_owned()),
            updated_at: Set(DateTimeWithTimeZone::from(Utc::now())),
            ..Default::default()
        }
        .update(db)
        .await
    }

    pub async fn delete_porduct_by_id(db: &DbConn, id: i32) -> Result<DeleteResult, DbErr> {
        let products: products::ActiveModel = Product::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find products.".to_owned()))
            .map(Into::into)?;
        products.delete(db).await
    }

    pub async fn get_porducts(db: &DbConn) -> Result<Vec<products::Model>, DbErr> {
        Product::find().all(db).await
    }

    // fenye
    pub async fn get_porducts_by_page(
        db: &DbConn,
        page: u64,
        size: u64,
    ) -> Result<(Vec<products::Model>, u64), DbErr> {
        let paginator = Product::find()
            .order_by_asc(products::Column::Id)
            .paginate(db, size);
        let num_pages = paginator.num_pages().await?;

        paginator.fetch_page(page - 1).await.map(|p| (p, num_pages))
    }

    pub async fn get_porduct_by_id(db: &DbConn, id: i32) -> Result<products::Model, DbErr> {
        Product::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find porduct.".to_owned()))
    }
}
