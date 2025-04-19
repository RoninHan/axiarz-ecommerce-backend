use ::entity::{home_page_product_type, home_page_product_type::Entity as HomePageProductType};
use chrono::{DateTime, Utc};
use prelude::DateTimeWithTimeZone;
use sea_orm::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct HomePageProductTypeModel {
    pub product_type_id: i32,
}

pub struct HomePageProductTypeServices;

impl HomePageProductTypeServices {
    pub async fn create_home_page_product_type(
        db: &DbConn,
        form_data: HomePageProductTypeModel,
    ) -> Result<home_page_product_type::ActiveModel, DbErr> {
        home_page_product_type::ActiveModel {
            product_type_id: Set(form_data.product_type_id),
            created_at: Set(Utc::now().into()),
            updated_at: Set(Utc::now().into()),
            ..Default::default()
        }
        .save(db)
        .await
    }

    pub async fn update_home_page_product_type_by_id(
        db: &DbConn,
        id: i32,
        form_data: HomePageProductTypeModel,
    ) -> Result<home_page_product_type::Model, DbErr> {
        let home_page_product_type: home_page_product_type::ActiveModel =
            HomePageProductType::find_by_id(id)
                .one(db)
                .await?
                .ok_or(DbErr::Custom(
                    "Cannot find home page product type.".to_owned(),
                ))
                .map(Into::into)?;

        home_page_product_type::ActiveModel {
            id: home_page_product_type.id,
            product_type_id: Set(form_data.product_type_id),
            updated_at: Set(Utc::now().into()),
            ..Default::default()
        }
        .update(db)
        .await
    }

    pub async fn delete_home_page_product_type_by_id(db: &DbConn, id: i32) -> Result<(), DbErr> {
        let home_page_product_type: home_page_product_type::ActiveModel =
            HomePageProductType::find_by_id(id)
                .one(db)
                .await?
                .ok_or(DbErr::Custom(
                    "Cannot find home page product type.".to_owned(),
                ))
                .map(Into::into)?;

        home_page_product_type.delete(db).await?;
        Ok(())
    }

    pub async fn get_home_page_product_type_all(
        db: &DbConn,
    ) -> Result<Vec<home_page_product_type::Model>, DbErr> {
        HomePageProductType::find().all(db).await
    }
}
