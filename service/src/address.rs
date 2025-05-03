use ::entity::{address, address::Entity as Address};
use chrono::{DateTime, Utc};
use prelude::DateTimeWithTimeZone;
use sea_orm::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct AddressModel {
    pub user_id: i32,
    pub phone: String,
    pub province: String,
    pub city: String,
    pub district: String,
    pub detail: String,
    pub postal_code: String,
}

pub struct AddressServices;

impl AddressServices {
    pub async fn create_address(
        db: &DbConn,
        form_data: AddressModel,
    ) -> Result<address::ActiveModel, DbErr> {
        address::ActiveModel {
            user_id: Set(form_data.user_id),
            phone: Set(form_data.phone),
            province: Set(form_data.province),
            city: Set(form_data.city),
            district: Set(form_data.district),
            detail: Set(form_data.detail),
            postal_code: Set(form_data.postal_code),
            created_at: Set(DateTimeWithTimeZone::from(Utc::now())),
            updated_at: Set(DateTimeWithTimeZone::from(Utc::now())),
            ..Default::default()
        }
        .save(db)
        .await
    }

    pub async fn update_address_by_id(
        db: &DbConn,
        id: i32,
        form_data: AddressModel,
    ) -> Result<address::Model, DbErr> {
        let address: address::ActiveModel = Address::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find address.".to_owned()))
            .map(Into::into)?;

        address::ActiveModel {
            id: address.id,
            user_id: Set(form_data.user_id),
            phone: Set(form_data.phone),
            province: Set(form_data.province),
            city: Set(form_data.city),
            district: Set(form_data.district),
            detail: Set(form_data.detail),
            postal_code: Set(form_data.postal_code),
            updated_at: Set(DateTimeWithTimeZone::from(Utc::now())),
            ..Default::default()
        }
        .update(db)
        .await
    }

    pub async fn delete_address_by_id(
        db: &DbConn,
        id: i32,
    ) -> Result<DeleteResult, DbErr> {
        let address: address::ActiveModel = Address::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find address.".to_owned()))
            .map(Into::into)?;

        address.delete(db).await
    }

    pub async fn get_addresses_by_user_id(
        db: &DbConn,
        user_id: i32,
    ) -> Result<Vec<address::Model>, DbErr> {
        Address::find()
            .filter(address::Column::UserId.eq(user_id))
            .all(db)
            .await
    }

    pub async fn get_address_by_id(
        db: &DbConn,
        id: i32,
    ) -> Result<Option<address::Model>, DbErr> {
        Address::find_by_id(id).one(db).await
    }
} 