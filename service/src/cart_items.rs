use ::entity::{cart_items, cart_items::Entity as CartItem};
use chrono::{DateTime, Utc};
use prelude::DateTimeWithTimeZone;
use sea_orm::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct CartItemModel {
    pub user_id: i32,
    pub product_id: i32,
    pub quantity: i32,
}

pub struct CartItemServices;

impl CartItemServices {
    pub async fn create_cart_item(
        db: &DbConn,
        form_data: CartItemModel,
    ) -> Result<cart_items::ActiveModel, DbErr> {
        cart_items::ActiveModel {
            user_id: Set(form_data.user_id),
            product_id: Set(form_data.product_id),
            quantity: Set(form_data.quantity),
            added_at: Set(Some(DateTimeWithTimeZone::from(Utc::now()))),
            ..Default::default()
        }
        .save(db)
        .await
    }

    pub async fn update_cart_item_by_id(
        db: &DbConn,
        id: i32,
        form_data: CartItemModel,
    ) -> Result<cart_items::Model, DbErr> {
        let cart_items: cart_items::ActiveModel = CartItem::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find cart_items.".to_owned()))
            .map(Into::into)?;
        cart_items::ActiveModel {
            id: cart_items.id,
            user_id: Set(form_data.user_id),
            product_id: Set(form_data.product_id),
            quantity: Set(form_data.quantity),
            ..Default::default()
        }
        .update(db)
        .await
    }

    pub async fn delete_cart_item_by_id(db: &DbConn, id: i32) -> Result<DeleteResult, DbErr> {
        let cart_items: cart_items::ActiveModel = CartItem::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find cart_items.".to_owned()))
            .map(Into::into)?;

        cart_items.delete(db).await
    }

    pub async fn get_cart_items_by_user_id(
        db: &DbConn,
        user_id: i32,
    ) -> Result<Vec<cart_items::Model>, DbErr> {
        CartItem::find()
            .filter(cart_items::Column::UserId.eq(user_id))
            .all(db)
            .await
    }
}
