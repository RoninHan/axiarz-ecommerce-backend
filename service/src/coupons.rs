use ::Entity::{coupons, coupons::Entity as Coupon};
use chrono::{DateTime, Utc};
use prelude::DateTimeWithTimeZone;
use sea_orm::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct CouponModel {
    pub code: String,
    pub discount: f64,
    pub valid_from: Option<DateTimeWithTimeZone>,
    pub valid_until: Option<DateTimeWithTimeZone>,
    pub usage_count: Option<i32>,
    pub total_count: Option<i32>,
}

pub struct CouponServices;

impl CouponServices {
    pub async fn create_coupon(
        db: &DbConn,
        form_data: CouponModel,
    ) -> Result<coupons::ActiveModel, DbErr> {
        coupons::ActiveModel {
            code: Set(form_data.code.to_owned()),
            discount: Set(form_data.discount),
            valid_from: Set(form_data.valid_from),
            valid_until: Set(form_data.valid_until),
            usage_count: Set(form_data.usage_count),
            total_count: Set(form_data.total_count),
            created_at: Set(DateTimeWithTimeZone::from(Utc::now())),
            ..Default::default()
        }
        .save(db)
        .await
    }

    pub async fn update_coupon_by_id(
        db: &DbConn,
        id: i32,
        form_data: CouponModel,
    ) -> Result<coupons::Model, DbErr> {
        let coupons: coupons::ActiveModel = Coupon::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find coupons.".to_owned()))
            .map(Into::into)?;
        coupons::ActiveModel {
            id: coupons.id,
            code: Set(form_data.code.to_owned()),
            discount: Set(form_data.discount),
            valid_from: Set(form_data.valid_from),
            valid_until: Set(form_data.valid_until),
            usage_count: Set(form_data.usage_count),
            total_count: Set(form_data.total_count),
            ..coupons
        }
        .save(db)
        .await
    }

    pub async fn delete_coupon_by_id(db: &DbConn, id: i32) -> Result<(), DbErr> {
        Coupon::delete()
            .filter(coupons::Column::Id.eq(id))
            .exec(db)
            .await?;
        Ok(())
    }

    pub async fn get_coupons(db: &DbConn) -> Result<Vec<coupons::Model>, DbErr> {
        Coupon::find().all(db).await
    }

    pub async fn get_coupon_by_id(db: &DbConn, id: i32) -> Result<coupons::Model, DbErr> {
        Coupon::find_by_id(id).one(db).await.ok_or(DbErr::NotFound)
    }

    pub async fn get_coupon_by_code(db: &DbConn, code: &str) -> Result<coupons::Model, DbErr> {
        Coupon::find()
            .filter(coupons::Column::Code.eq(code))
            .one(db)
            .await
            .ok_or(DbErr::NotFound)
    }
}
