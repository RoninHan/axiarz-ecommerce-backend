use ::entity::{orders, orders::Entity as Order};
use chrono::{DateTime, Utc};
use prelude::DateTimeWithTimeZone;
use sea_orm::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct OrderModel {
    pub user_id: i32,
    pub total_price: prelude::Decimal,
    pub status: i32,
    pub shipping_status: i32,
    pub shipping_company: Option<String>,
    pub tracking_number: Option<String>,
    pub payment_status: i32,
    pub payment_method: i32,
    pub shipping_address: String,
    pub billing_address: String,
    pub discount: Option<prelude::Decimal>,
    pub coupon_code: Option<String>,
    pub gift_card_code: Option<String>,
    pub notes: Option<String>,
}

pub struct OrderServices;

impl OrderServices {
    pub async fn create_order(
        db: &DbConn,
        form_data: OrderModel,
    ) -> Result<orders::ActiveModel, DbErr> {
        orders::ActiveModel {
            user_id: Set(form_data.user_id),
            total_price: Set(form_data.total_price),
            status: Set(form_data.status.to_owned()),
            shipping_status: Set(form_data.shipping_status),
            shipping_company: Set(form_data.shipping_company.to_owned()),
            tracking_number: Set(form_data.tracking_number.to_owned()),
            payment_status: Set(form_data.payment_status),
            payment_method: Set(form_data.payment_method),
            shipping_address: Set(form_data.shipping_address.to_owned()),
            billing_address: Set(form_data.billing_address.to_owned()),
            discount: Set(form_data.discount),
            coupon_code: Set(form_data.coupon_code.to_owned()),
            gift_card_code: Set(form_data.gift_card_code.to_owned()),
            notes: Set(form_data.notes.to_owned()),
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
            total_price: Set(form_data.total_price.to_owned()),
            status: Set(form_data.status.to_owned()),
            shipping_status: Set(form_data.shipping_status),
            shipping_company: Set(form_data.shipping_company.to_owned()),
            tracking_number: Set(form_data.tracking_number.to_owned()),
            payment_status: Set(form_data.payment_status),
            payment_method: Set(form_data.payment_method),
            shipping_address: Set(form_data.shipping_address.to_owned()),
            billing_address: Set(form_data.billing_address.to_owned()),
            discount: Set(form_data.discount),
            coupon_code: Set(form_data.coupon_code.to_owned()),
            gift_card_code: Set(form_data.gift_card_code.to_owned()),
            notes: Set(form_data.notes.to_owned()),
            updated_at: Set(DateTimeWithTimeZone::from(Utc::now())),
            ..Default::default()
        }
        .update(db)
        .await
    }

    pub async fn delete_order_by_id(db: &DbConn, id: i32) -> Result<DeleteResult, DbErr> {
        let orders: orders::ActiveModel = Order::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find orders.".to_owned()))
            .map(Into::into)?;

        orders.delete(db).await
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
        Order::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find order.".to_owned()))
    }

    // 分页
    pub async fn get_orders(
        db: &DbConn,
        page: u64,
        limit: u64,
    ) -> Result<(Vec<orders::Model>, u64), DbErr> {
        let paginator = Order::find()
            .order_by_asc(orders::Column::Id)
            .paginate(db, limit);

        let num_pages = paginator.num_pages().await?;

        paginator.fetch_page(page - 1).await.map(|p| (p, num_pages))
    }

    // 设置支付状态
    pub async fn set_payment_status(
        db: &DbConn,
        id: i32,
        payment_status: i32,
    ) -> Result<orders::Model, DbErr> {
        let orders: orders::ActiveModel = Order::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find orders.".to_owned()))
            .map(Into::into)?;

        orders::ActiveModel {
            id: orders.id,
            payment_status: Set(payment_status),
            paid_at: Set(Some(DateTimeWithTimeZone::from(Utc::now()))),
            updated_at: Set(DateTimeWithTimeZone::from(Utc::now())),
            ..Default::default()
        }
        .update(db)
        .await
    }

    // 设置支付方式
    pub async fn set_payment_method(
        db: &DbConn,
        id: i32,
        payment_method: i32,
    ) -> Result<orders::Model, DbErr> {
        let orders: orders::ActiveModel = Order::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find orders.".to_owned()))
            .map(Into::into)?;

        orders::ActiveModel {
            id: orders.id,
            payment_method: Set(payment_method),
            updated_at: Set(DateTimeWithTimeZone::from(Utc::now())),
            ..Default::default()
        }
        .update(db)
        .await
    }

    // 设置物流信息
    pub async fn set_shipping_status(
        db: &DbConn,
        id: i32,
        shipping_company: String,
        tracking_number: String,
    ) -> Result<orders::Model, DbErr> {
        let orders: orders::ActiveModel = Order::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find orders.".to_owned()))
            .map(Into::into)?;

        orders::ActiveModel {
            id: orders.id,
            shipping_status: Set(1),
            shipping_company: Set(Some(shipping_company)),
            tracking_number: Set(Some(tracking_number)),
            updated_at: Set(DateTimeWithTimeZone::from(Utc::now())),
            ..Default::default()
        }
        .update(db)
        .await
    }

    // 设置订单状态
    // 'pending'   // 待处理
    // 'paid'      // 已付款
    // 'shipped'   // 已发货
    // 'completed' // 已完成
    // 'canceled'  // 已取消
    // 'refunded'  // 已退款
    pub async fn set_order_status(
        db: &DbConn,
        id: i32,
        status: i32,
    ) -> Result<orders::Model, DbErr> {
        let orders: orders::ActiveModel = Order::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find orders.".to_owned()))
            .map(Into::into)?;

        orders::ActiveModel {
            id: orders.id,
            status: Set(status),
            updated_at: Set(DateTimeWithTimeZone::from(Utc::now())),
            ..Default::default()
        }
        .update(db)
        .await
    }
}
