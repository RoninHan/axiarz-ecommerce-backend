use ::Entity::{refunds, refunds::Entity as Refund};
use chrono::{DateTime, Utc};
use prelude::DateTimeWithTimeZone;
use sea_orm::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct RefundModel {
    pub payment_id: i32,
    pub refund_amount: f64,
    pub refund_status: i32,
    pub refund_reason: Option<String>,
    pub refund_requested_at: Option<DateTimeWithTimeZone>,
    pub refund_processed_at: Option<DateTimeWithTimeZone>,
}

pub struct RefundServices;

impl RefundServices {
    pub async fn create_refund(
        db: &DbConn,
        form_data: RefundModel,
    ) -> Result<refunds::ActiveModel, DbErr> {
        refunds::ActiveModel {
            payment_id: Set(form_data.payment_id),
            refund_amount: Set(form_data.refund_amount),
            refund_status: Set(form_data.refund_status),
            refund_reason: Set(form_data.refund_reason),
            refund_requested_at: Set(form_data.refund_requested_at),
            refund_processed_at: Set(form_data.refund_processed_at),
            created_at: Set(DateTimeWithTimeZone::from(Utc::now())),
            updated_at: Set(DateTimeWithTimeZone::from(Utc::now())),
            ..Default::default()
        }
        .save(db)
        .await
    }

    pub async fn update_refund_by_id(
        db: &DbConn,
        id: i32,
        form_data: RefundModel,
    ) -> Result<refunds::Model, DbErr> {
        let refunds: refunds::ActiveModel = Refund::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find refunds.".to_owned()))
            .map(Into::into)?;
        refunds::ActiveModel {
            id: refunds.id,
            payment_id: Set(form_data.payment_id),
            refund_amount: Set(form_data.refund_amount),
            refund_status: Set(form_data.refund_status),
            refund_reason: Set(form_data.refund_reason),
            refund_requested_at: Set(form_data.refund_requested_at),
            refund_processed_at: Set(form_data.refund_processed_at),
            ..refunds
        }
        .save(db)
        .await
    }

    pub async fn delete_refund_by_id(db: &DbConn, id: i32) -> Result<(), DbErr> {
        Refund::delete()
            .filter(refunds::Column::Id.eq(id))
            .exec(db)
            .await?;
        Ok(())
    }

    pub async fn get_refunds_by_payment_id(
        db: &DbConn,
        payment_id: i32,
    ) -> Result<Vec<refunds::Model>, DbErr> {
        Refund::find()
            .filter(refunds::Column::PaymentId.eq(payment_id))
            .all(db)
            .await
    }

    pub async fn get_refund_by_id(db: &DbConn, id: i32) -> Result<refunds::Model, DbErr> {
        Refund::find_by_id(id).one(db).await
    }

    pub async fn get_all_refunds(db: &DbConn) -> Result<Vec<refunds::Model>, DbErr> {
        let refunds: Vec<refunds::Model> = Refund::find().all(db).await?;
        Ok(refunds)
    }

    // 分頁
    pub async fn get_refunds_by_page(
        db: &DbConn,
        page: i32,
        size: i32,
    ) -> Result<Vec<refunds::Model>, DbErr> {
        let refunds: Vec<refunds::Model> = Refund::find().paginate(page, size).all(db).await?;
        Ok(refunds)
    }
}
