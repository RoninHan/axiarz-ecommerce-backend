use ::Entity::{payments, payments::Entity as Payment};
use chrono::{DateTime, Utc};
use prelude::DateTimeWithTimeZone;
use sea_orm::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct PaymentModel {
    pub order_id: i32,
    pub payment_method: i32,
    pub transaction_id: String,
    pub pay_status: i32,
    pub amount: Decimal,
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
            ..payments
        }
        .save(db)
        .await
    }

    pub async fn delete_payment_by_id(db: &DbConn, id: i32) -> Result<(), DbErr> {
        Payment::delete()
            .filter(payments::Column::Id.eq(id))
            .exec(db)
            .await?;
        Ok(())
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
        Payment::find_by_id(id).one(db).await
    }

    // 分页查询
    pub async fn get_payments(
        db: &DbConn,
        page: i32,
        size: i32,
    ) -> Result<Vec<payments::Model>, DbErr> {
        Payment::find().paginate(page, size).all(db).await
    }
}
