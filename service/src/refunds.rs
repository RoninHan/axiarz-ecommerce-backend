use ::entity::{refunds, refunds::Entity as Refund};
use chrono::{DateTime, Utc};
use prelude::DateTimeWithTimeZone;
use sea_orm::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct RefundModel {
    pub payment_id: i32,
    pub refund_amount: prelude::Decimal,
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
        }
        .update(db)
        .await
    }

    pub async fn delete_refund_by_id(db: &DbConn, id: i32) -> Result<(), DbErr> {
        let refunds: refunds::ActiveModel = Refund::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find refunds.".to_owned()))
            .map(Into::into)?;
        refunds.delete(db).await?;
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
        Refund::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find refund.".to_owned()))
    }

    pub async fn get_all_refunds(db: &DbConn) -> Result<Vec<refunds::Model>, DbErr> {
        let refunds: Vec<refunds::Model> = Refund::find().all(db).await?;
        Ok(refunds)
    }

    // 分頁
    pub async fn get_refunds_by_page(
        db: &DbConn,
        page: u64,
        size: u64,
    ) -> Result<(Vec<refunds::Model>, u64), DbErr> {
        let paginator = Refund::find()
            .order_by_asc(refunds::Column::Id)
            .paginate(db, size);
        let num_pages = paginator.num_pages().await?;

        paginator.fetch_page(page - 1).await.map(|p| (p, num_pages))
    }
}
