use ::entity::{payments, payments::Entity as Payment};
use chrono::{DateTime, Utc};
use prelude::DateTimeWithTimeZone;
use sea_orm::*;

use serde::{Deserialize, Serialize};

#[derive(Debug, serde::Deserialize)]
pub struct RequestCreatePaymentBody {
    pub order_id: i32,
    pub payment_method: i32,
    pub amount: i32,
    pub description: String,
    pub open_id: String,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct PaymentModel {
    pub order_id: i32,
    pub payment_method: i32,
    pub transaction_id: String,
    pub pay_status: i32,
    pub amount: prelude::Decimal,
    pub paid_at: Option<DateTimeWithTimeZone>,
}

pub struct PaymentServices;

impl PaymentServices {
    pub async fn create_payment(
        db: &DbConn,
        form_data: PaymentModel,
    ) -> Result<payments::ActiveModel, DbErr> {
        payments::ActiveModel {
            order_id: Set(form_data.order_id),
            payment_method: Set(form_data.payment_method),
            transaction_id: Set(form_data.transaction_id.to_owned()),
            pay_status: Set(form_data.pay_status),
            amount: Set(form_data.amount),
            paid_at: Set(form_data.paid_at),
            created_at: Set(DateTimeWithTimeZone::from(Utc::now())),
            updated_at: Set(DateTimeWithTimeZone::from(Utc::now())),
            ..Default::default()
        }
        .save(db)
        .await
    }

    pub async fn update_payment_by_id(
        db: &DbConn,
        id: i32,
        form_data: PaymentModel,
    ) -> Result<payments::Model, DbErr> {
        let payments: payments::ActiveModel = Payment::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find payments.".to_owned()))
            .map(Into::into)?;
        payments::ActiveModel {
            id: payments.id,
            order_id: Set(form_data.order_id),
            payment_method: Set(form_data.payment_method),
            transaction_id: Set(form_data.transaction_id.to_owned()),
            pay_status: Set(form_data.pay_status),
            amount: Set(form_data.amount),
            paid_at: Set(form_data.paid_at),
            updated_at: Set(DateTimeWithTimeZone::from(Utc::now())),
            ..Default::default()
        }
        .update(db)
        .await
    }

    pub async fn update_payment_status(
        db: &DbConn,
        order_id: i32,
        pay_status: i32,
        paid_at: Option<DateTime<Utc>>,
    ) -> Result<payments::Model, DbErr> {
        let payment = Payment::find()
            .filter(payments::Column::OrderId.eq(order_id))
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find payment.".to_owned()))?;

        let mut payment: payments::ActiveModel = payment.into();
        payment.pay_status = Set(pay_status);
        payment.paid_at = Set(paid_at.map(DateTimeWithTimeZone::from));
        payment.updated_at = Set(DateTimeWithTimeZone::from(Utc::now()));

        payment.update(db).await
    }

    pub async fn delete_payment_by_id(db: &DbConn, id: i32) -> Result<DeleteResult, DbErr> {
        let payments: payments::ActiveModel = Payment::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find payments.".to_owned()))
            .map(Into::into)?;

        payments.delete(db).await
    }

    pub async fn get_payments_by_order_id(
        db: &DbConn,
        order_id: i32,
    ) -> Result<Vec<payments::Model>, DbErr> {
        Payment::find()
            .filter(payments::Column::OrderId.eq(order_id))
            .all(db)
            .await
    }

    pub async fn get_payment_by_id(db: &DbConn, id: i32) -> Result<payments::Model, DbErr> {
        Payment::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find payment.".to_owned()))
    }

    // 分页查询
    pub async fn get_payments(
        db: &DbConn,
        page: u64,
        size: u64,
    ) -> Result<(Vec<payments::Model>, u64), DbErr> {
        let paginator = Payment::find()
            .order_by_asc(payments::Column::Id)
            .paginate(db, size);
        let num_pages = paginator.num_pages().await?;

        paginator.fetch_page(page - 1).await.map(|p| (p, num_pages))
    }
}
