use ::entity::{shipping_info, shipping_info::Entity as ShippingInfo};

use chrono::{DateTime, Utc};

use prelude::DateTimeWithTimeZone;

use sea_orm::*;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct ShippingInfoModel {
    pub order_id: i32,
    pub shipping_company: String,
    pub tracking_number: String,
    pub shipping_status: i32,
    pub estimated_delivery_date: Option<DateTimeWithTimeZone>,
    pub shipped_at: Option<DateTimeWithTimeZone>,
    pub delivered_at: Option<DateTimeWithTimeZone>,
}

pub struct ShippingInfoServices;

impl ShippingInfoServices {
    pub async fn create_shipping_info(
        db: &DbConn,
        form_data: ShippingInfoModel,
    ) -> Result<shipping_info::ActiveModel, DbErr> {
        shipping_info::ActiveModel {
            order_id: Set(form_data.order_id),
            shipping_company: Set(form_data.shipping_company.to_owned()),
            tracking_number: Set(form_data.tracking_number.to_owned()),
            shipping_status: Set(form_data.shipping_status),
            estimated_delivery_date: Set(form_data.estimated_delivery_date),
            shipped_at: Set(form_data.shipped_at),
            delivered_at: Set(form_data.delivered_at),
            created_at: Set(DateTimeWithTimeZone::from(Utc::now())),
            updated_at: Set(DateTimeWithTimeZone::from(Utc::now())),
            ..Default::default()
        }
        .save(db)
        .await
    }

    pub async fn update_shipping_info_by_id(
        db: &DbConn,
        id: i32,
        form_data: ShippingInfoModel,
    ) -> Result<shipping_info::Model, DbErr> {
        let shipping_info: shipping_info::ActiveModel = ShippingInfo::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find shipping_info.".to_owned()))
            .map(Into::into)?;
        shipping_info::ActiveModel {
            id: shipping_info.id,
            order_id: Set(form_data.order_id),
            shipping_company: Set(form_data.shipping_company.to_owned()),
            tracking_number: Set(form_data.tracking_number.to_owned()),
            shipping_status: Set(form_data.shipping_status),
            estimated_delivery_date: Set(form_data.estimated_delivery_date),
            shipped_at: Set(form_data.shipped_at),
            delivered_at: Set(form_data.delivered_at),
            updated_at: Set(DateTimeWithTimeZone::from(Utc::now())),
            ..Default::default()
        }
        .update(db)
        .await
    }

    pub async fn delete_shipping_info_by_id(db: &DbConn, id: i32) -> Result<(), DbErr> {
        let shipping_info: shipping_info::ActiveModel = ShippingInfo::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find shipping_info.".to_owned()))
            .map(Into::into)?;

            shipping_info.delete(db).await?;
            Ok(())
    }

    pub async fn get_shipping_info_by_order_id(
        db: &DbConn,
        order_id: i32,
    ) -> Result<Vec<shipping_info::Model>, DbErr> {
        ShippingInfo::find()
            .filter(shipping_info::Column::OrderId.eq(order_id))
            .all(db)
            .await
    }

    pub async fn get_shipping_info_by_id(
        db: &DbConn,
        id: i32,
    ) -> Result<shipping_info::Model, DbErr> {
        ShippingInfo::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Shipping info not found.".to_owned()))
    }

    // 分頁
    pub async fn get_shipping_info_by_order_id_page(
        db: &DbConn,
        order_id: i32,
        page: u64,
        page_size: u64,
    ) -> Result<(Vec<shipping_info::Model>, u64), DbErr> {
        let paginator=ShippingInfo::find()
        .order_by_asc(shipping_info::Column::Id)
        .paginate(db, page_size);

        let num_pages = paginator.num_pages().await?;

        paginator.fetch_page(page - 1).await.map(|p| (p, num_pages))

    }

    pub async fn get_all_shipping_info(db: &DbConn) -> Result<Vec<shipping_info::Model>, DbErr> {
        ShippingInfo::find().all(db).await
    }
}
