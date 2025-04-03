use ::entity::{banner, banner::Entity as BannerEntity};
use chrono::{DateTime, Utc};
use prelude::DateTimeWithTimeZone;
use sea_orm::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct BannerModel {
    pub title: String,
    pub image_url: String,
    pub link: String,
}

pub struct BannerServices;

impl BannerServices {
    pub async fn create_banner(
        db: &DbConn,
        form_data: BannerModel,
    ) -> Result<banner::ActiveModel, DbErr> {
        banner::ActiveModel {
            title: Set(form_data.title.to_owned()),
            image_url: Set(form_data.image_url.to_owned()),
            link: Set(form_data.link.to_owned()),
            created_at: Set(DateTimeWithTimeZone::from(Utc::now())),
            updated_at: Set(DateTimeWithTimeZone::from(Utc::now())),
            ..Default::default()
        }
        .save(db)
        .await
    }

    pub async fn update_banner_by_id(
        db: &DbConn,
        id: i32,
        form_data: BannerModel,
    ) -> Result<banner::Model, DbErr> {
        let banner: banner::ActiveModel = BannerEntity::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find banner.".to_owned()))
            .map(Into::into)?;

        banner::ActiveModel {
            id: banner.id,
            title: Set(form_data.title.to_owned()),
            image_url: Set(form_data.image_url.to_owned()),
            link: Set(form_data.link.to_owned()),
            updated_at: Set(DateTimeWithTimeZone::from(Utc::now())),
            ..Default::default()
        }
        .update(db)
        .await
    }

    pub async fn delete_banner_by_id(db: &DbConn, id: i32) -> Result<(), DbErr> {
        let banner: banner::ActiveModel = BannerEntity::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find banner.".to_owned()))
            .map(Into::into)?;

        let _ = banner.delete(db).await;
        Ok(())
    }

    pub async fn get_banner_all(db: &DbConn) -> Result<Vec<banner::Model>, DbErr> {
        BannerEntity::find().all(db).await
    }

    pub async fn get_banner_by_id(db: &DbConn, id: i32) -> Result<banner::Model, DbErr> {
        BannerEntity::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find banner.".to_owned()))
    }
}
