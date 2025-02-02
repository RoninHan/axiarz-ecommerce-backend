use ::Entity::{order_items, order_items::Entity as OrderItem};
use chrono::{DateTime, Utc};
use prelude::DateTimeWithTimeZone;
use sea_orm::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct OrderItemModel {
    pub order_id: i32,
    pub product_id: i32,
    pub quantity: i32,
    pub price: Decimal,
}

pub struct OrderItemServices;

impl OrderItemServices {
    pub async fn create_order_item(
        db: &DbConn,
        form_data: OrderItemModel,
    ) -> Result<order_items::ActiveModel, DbErr> {
        order_items::ActiveModel {
            order_id: Set(form_data.order_id),
            product_id: Set(form_data.product_id),
            quantity: Set(form_data.quantity),
            price: Set(form_data.price),
            created_at: Set(DateTimeWithTimeZone::from(Utc::now())),
            updated_at: Set(DateTimeWithTimeZone::from(Utc::now())),
            ..Default::default()
        }
        .save(db)
        .await
    }

    pub async fn update_order_item_by_id(
        db: &DbConn,
        id: i32,
        form_data: OrderItemModel,
    ) -> Result<order_items::Model, DbErr> {
        let order_items: order_items::ActiveModel = OrderItem::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find order_items.".to_owned()))
            .map(Into::into)?;
        order_items::ActiveModel {
            id: order_items.id,
            order_id: Set(form_data.order_id),
            product_id: Set(form_data.product_id),
            quantity: Set(form_data.quantity),
            price: Set(form_data.price),
            ..order_items
        }
        .save(db)
        .await
    }

    pub async fn delete_order_item_by_id(db: &DbConn, id: i32) -> Result<(), DbErr> {
        OrderItem::delete()
            .filter(order_items::Column::Id.eq(id))
            .exec(db)
            .await?;
        Ok(())
    }

    pub async fn get_order_items_by_order_id(
        db: &DbConn,
        order_id: i32,
    ) -> Result<Vec<order_items::Model>, DbErr> {
        OrderItem::find()
            .filter(order_items::Column::OrderId.eq(order_id))
            .all(db)
            .await
    }

    pub async fn get_order_item_by_id(db: &DbConn, id: i32) -> Result<order_items::Model, DbErr> {
        OrderItem::find_by_id(id).one(db).await
    }

    pub async fn get_order_items(db: &DbConn) -> Result<Vec<order_items::Model>, DbErr> {
        OrderItem::find().all(db).await
    }

    pub async fn get_order_items_by_product_id(
        db: &DbConn,
        product_id: i32,
    ) -> Result<Vec<order_items::Model>, DbErr> {
        OrderItem::find()
            .filter(order_items::Column::ProductId.eq(product_id))
            .all(db)
            .await
    }
}
