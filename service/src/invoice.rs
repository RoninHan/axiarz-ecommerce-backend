use ::entity::{invoice, invoice::Entity as Invoice};
use chrono::{DateTime, Utc};
use prelude::DateTimeWithTimeZone;
use sea_orm::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct InvoiceModel {
    pub user_id: i32,
    pub r#type: i32,
    pub title: String,
    pub tax_number: Option<String>,
    pub content: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub is_default: bool,
}

pub struct InvoiceServices;

impl InvoiceServices {
    pub async fn create_invoice(
        db: &DbConn,
        form_data: InvoiceModel,
    ) -> Result<invoice::ActiveModel, DbErr> {
        // 如果设置为默认发票，需要先将该用户的其他发票设置为非默认
        if form_data.is_default {
            Invoice::update_many()
                .set(invoice::ActiveModel {
                    is_default: Set(false),
                    ..Default::default()
                })
                .filter(invoice::Column::UserId.eq(form_data.user_id))
                .exec(db)
                .await?;
        }

        invoice::ActiveModel {
            user_id: Set(form_data.user_id),
            r#type: Set(form_data.r#type),
            title: Set(form_data.title),
            tax_number: Set(form_data.tax_number),
            content: Set(form_data.content),
            email: Set(form_data.email),
            phone: Set(form_data.phone),
            is_default: Set(form_data.is_default),
            created_at: Set(DateTimeWithTimeZone::from(Utc::now())),
            updated_at: Set(DateTimeWithTimeZone::from(Utc::now())),
            ..Default::default()
        }
        .save(db)
        .await
    }

    pub async fn update_invoice_by_id(
        db: &DbConn,
        id: i32,
        form_data: InvoiceModel,
    ) -> Result<invoice::Model, DbErr> {
        // 如果设置为默认发票，需要先将该用户的其他发票设置为非默认
        if form_data.is_default {
            Invoice::update_many()
                .set(invoice::ActiveModel {
                    is_default: Set(false),
                    ..Default::default()
                })
                .filter(invoice::Column::UserId.eq(form_data.user_id))
                .exec(db)
                .await?;
        }

        let invoice: invoice::ActiveModel = Invoice::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find invoice.".to_owned()))
            .map(Into::into)?;

        invoice::ActiveModel {
            id: invoice.id,
            user_id: Set(form_data.user_id),
            r#type: Set(form_data.r#type),
            title: Set(form_data.title),
            tax_number: Set(form_data.tax_number),
            content: Set(form_data.content),
            email: Set(form_data.email),
            phone: Set(form_data.phone),
            is_default: Set(form_data.is_default),
            updated_at: Set(DateTimeWithTimeZone::from(Utc::now())),
            ..Default::default()
        }
        .update(db)
        .await
    }

    pub async fn delete_invoice_by_id(
        db: &DbConn,
        id: i32,
    ) -> Result<DeleteResult, DbErr> {
        let invoice: invoice::ActiveModel = Invoice::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find invoice.".to_owned()))
            .map(Into::into)?;

        invoice.delete(db).await
    }

    pub async fn get_invoices_by_user_id(
        db: &DbConn,
        user_id: i32,
    ) -> Result<Vec<invoice::Model>, DbErr> {
        Invoice::find()
            .filter(invoice::Column::UserId.eq(user_id))
            .all(db)
            .await
    }

    pub async fn get_invoice_by_id(
        db: &DbConn,
        id: i32,
    ) -> Result<Option<invoice::Model>, DbErr> {
        Invoice::find_by_id(id).one(db).await
    }

    pub async fn get_default_invoice_by_user_id(
        db: &DbConn,
        user_id: i32,
    ) -> Result<Option<invoice::Model>, DbErr> {
        Invoice::find()
            .filter(invoice::Column::UserId.eq(user_id))
            .filter(invoice::Column::IsDefault.eq(true))
            .one(db)
            .await
    }
} 