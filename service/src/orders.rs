use ::Entity::{orders, orders::Entity as Order};
use chrono::{DateTime, Utc};
use prelude::DateTimeWithTimeZone;
use sea_orm::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct OrderModel {
    pub user_id: i32,
    pub total: i32,
    pub status: String,
}

pub struct OrderServices;

impl OrderServices {
    pub async fn create_order(
        db: &DbConn,
        form_data: OrderModel,
    ) -> Result<orders::ActiveModel, DbErr> {
        orders::ActiveModel {
            user_id: Set(form_data.user_id),
            total: Set(form_data.total),
            status: Set(form_data.status.to_owned()),
            created_at: Set(DateTimeWithTimeZone::from(Utc::now())),
            updated_at: Set(DateTimeWithTimeZone::from(Utc::now())),
            ..Default::default()
        }
        .save(db)
        .await
    }

    pub async fn update_order_by_id(
        db: &DbConn,
        id: i32,
        form_data: OrderModel,
    ) -> Result<orders::Model, DbErr> {
        let orders: orders::ActiveModel = Order::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find orders.".to_owned()))
            .map(Into::into)?;
        orders::ActiveModel {
            id: orders.id,
            user_id: Set(form_data.user_id),
            total: Set(form_data.total),
            status: Set(form_data.status.to_owned()),
            ..orders
        }
        .save(db)
        .await
    }

    pub async fn delete_order_by_id(db: &DbConn, id: i32) -> Result<(), DbErr> {
        Order::delete()
            .filter(orders::Column::Id.eq(id))
            .exec(db)
            .await?;
        Ok(())
    }

    pub async fn get_orders_by_user_id(
        db: &DbConn,
        user_id: i32,
    ) -> Result<Vec<orders::Model>, DbErr> {
        Order::find()
            .filter(orders::Column::UserId.eq(user_id))
            .all(db)
            .await
    }

    pub async fn get_order_by_id(db: &DbConn, id: i32) -> Result<orders::Model, DbErr> {
        Order::find_by_id(id).one(db).await
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

    // 分页
    pub async fn get_orders(
        db: &DbConn,
        page: i32,
        limit: i32,
    ) -> Result<Vec<orders::Model>, DbErr> {
        Order::find().paginate(page).per_page(limit).all(db).await
    }
}
