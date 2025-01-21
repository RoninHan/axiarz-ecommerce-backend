use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;


#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(pk_auto(Users::Id))
                    .col(ColumnDef::new(Users::Name).string().not_null())
                    .col(ColumnDef::new(Users::Sex).integer().not_null())
                    .col(ColumnDef::new(Users::Password).string().not_null())
                    .col(ColumnDef::new(Users::Birthday).timestamp_with_time_zone())
                    .col(ColumnDef::new(Users::Phone).string())
                    .col(ColumnDef::new(Users::Email).string())
                    .col(
                        ColumnDef::new(Users::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Users::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Porducts::Table)
                    .if_not_exists()
                    .col(pk_auto(Porducts::Id))
                    .col(ColumnDef::new(Porducts::Name).string().not_null())
                    .col(ColumnDef::new(Porducts::Description).string())
                    .col(ColumnDef::new(Porducts::StockQuantity).integer().not_null())
                    .col(ColumnDef::new(Porducts::Price).decimal().not_null())
                    .col(ColumnDef::new(Porducts::ImageUrl).string())
                    .col(
                        ColumnDef::new(Porducts::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Porducts::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Orders::Table)
                    .if_not_exists()
                    .col(pk_auto(Orders::Id))
                    .col(ColumnDef::new(Orders::UserId).integer().not_null())
                    .col(ColumnDef::new(Orders::TotalPrice).decimal().not_null())
                    .col(ColumnDef::new(Orders::Status).enumeration(Orders::Status,vec![
                        OrdersStatus::Pending,
                        OrdersStatus::Paid,
                        OrdersStatus::Shipped,
                        OrdersStatus::Completed,
                        OrdersStatus::Canceled,
                        OrdersStatus::Refunded,
                    ]))
                    .col(ColumnDef::new(Orders::ShippingStatus).enumeration(Orders::ShippingStatus,vec![
                        ShippingStatus::Pending,
                                    ShippingStatus::Shipped,
                                    ShippingStatus::Delivered,
                                    ShippingStatus::Cancelled,
                    ]))
                    .col(ColumnDef::new(Orders::ShippingCompany).string())
                    .col(ColumnDef::new(Orders::TrackingNumber).string())
                    .col(
                        ColumnDef::new(Orders::PaymentStatus)
                            .enumeration(Orders::PaymentStatus, vec![
                                PaymentStatus::Pending,
                                PaymentStatus::Paid,
                                PaymentStatus::Failed,
                                PaymentStatus::Refunded,
                            ]),
                    )
                    .col(ColumnDef::new(Orders::PaymentMethod).enumeration(Orders::PaymentMethod, vec![
                        PaymentMethod::Wechat,
                        PaymentMethod::Alipay,
                        PaymentMethod::CreditCard,
                        PaymentMethod::Paypal,
                        PaymentMethod::BankTransfer,
                    ]))
                    .col(
                        ColumnDef::new(Orders::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Orders::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Orders::PaidAt).timestamp_with_time_zone())
                    .col(ColumnDef::new(Orders::ShippedAt).timestamp_with_time_zone())
                    .col(ColumnDef::new(Orders::DeliveredAt).timestamp_with_time_zone())
                    .col(ColumnDef::new(Orders::CanceledAt).timestamp_with_time_zone())
                    .col(ColumnDef::new(Orders::RefundedAt).timestamp_with_time_zone())
                    .col(ColumnDef::new(Orders::ShippingAddress).text().not_null())
                    .col(ColumnDef::new(Orders::BillingAddress).text().not_null())
                    .col(ColumnDef::new(Orders::Discount).decimal().default("0.00"))
                    .col(ColumnDef::new(Orders::CouponCode).string())
                    .col(ColumnDef::new(Orders::GiftCardCode).string())
                    .col(ColumnDef::new(Orders::Notes).text())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(OrderItems::Table)
                    .if_not_exists()
                    .col(pk_auto(OrderItems::Id))
                    .col(ColumnDef::new(OrderItems::OrderId).integer().not_null())
                    .col(ColumnDef::new(OrderItems::ProductId).integer().not_null())
                    .col(ColumnDef::new(OrderItems::Quantity).integer().not_null())
                    .col(ColumnDef::new(OrderItems::Price).decimal().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Payments::Table)
                    .if_not_exists()
                    .col(pk_auto(Payments::Id))
                    .col(ColumnDef::new(Payments::OrderId).integer().not_null())
                    .col(ColumnDef::new(Payments::PaymentMethod).enumeration(Payments::PaymentMethod, vec![
                        PaymentMethod::Wechat,
                        PaymentMethod::Alipay,
                    ]))
                    .col(ColumnDef::new(Payments::TransactionId).string().not_null())
                    .col(
                        ColumnDef::new(Payments::PayStatus)
                            .enumeration(Payments::PayStatus, vec![
                                PaymentStatus::Pending,
                                PaymentStatus::Paid,
                                PaymentStatus::Failed,
                                PaymentStatus::Refunded,
                            ]),
                    )
                    .col(ColumnDef::new(Payments::Amount).decimal().not_null())
                    .col(ColumnDef::new(Payments::PaidAt).timestamp_with_time_zone())
                    .col(
                        ColumnDef::new(Payments::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Payments::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(CartItems::Table)
                    .if_not_exists()
                    .col(pk_auto(CartItems::Id))
                    .col(ColumnDef::new(CartItems::UserId).integer().not_null())
                    .col(ColumnDef::new(CartItems::ProductId).integer().not_null())
                    .col(ColumnDef::new(CartItems::Quantity).integer().not_null())
                    .col(
                        ColumnDef::new(CartItems::AddedAt)
                            .timestamp_with_time_zone()
                            .default("CURRENT_TIMESTAMP"),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(ShippingInfo::Table)
                    .if_not_exists()
                    .col(pk_auto(ShippingInfo::Id))
                    .col(ColumnDef::new(ShippingInfo::OrderId).integer().not_null())
                    .col(ColumnDef::new(ShippingInfo::ShippingCompany).string().not_null())
                    .col(ColumnDef::new(ShippingInfo::TrackingNumber).string().not_null())
                    .col(ColumnDef::new(ShippingInfo::ShippingStatus).enumeration(ShippingInfo::ShippingStatus, vec![
                        ShippingStatus::Pending,
                        ShippingStatus::Shipped,
                        ShippingStatus::Delivered,
                        ShippingStatus::Cancelled,
                    ]))
                    .col(ColumnDef::new(ShippingInfo::EstimatedDeliveryDate).date())
                    .col(ColumnDef::new(ShippingInfo::ShippedAt).timestamp_with_time_zone())
                    .col(ColumnDef::new(ShippingInfo::DeliveredAt).timestamp_with_time_zone())
                    .col(
                        ColumnDef::new(ShippingInfo::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ShippingInfo::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Coupons::Table)
                    .if_not_exists()
                    .col(pk_auto(Coupons::Id))
                    .col(ColumnDef::new(Coupons::Code).string().not_null())
                    .col(ColumnDef::new(Coupons::Discount).decimal().not_null())
                    .col(ColumnDef::new(Coupons::ValidFrom).timestamp_with_time_zone())
                    .col(ColumnDef::new(Coupons::ValidUntil).timestamp_with_time_zone())
                    .col(ColumnDef::new(Coupons::UsageCount).integer().default("0"))
                    .col(ColumnDef::new(Coupons::TotalCount).integer().default("0"))
                    .col(
                        ColumnDef::new(Coupons::CreatedAt)
                            .timestamp_with_time_zone()
                            .default("CURRENT_TIMESTAMP"),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Categories::Table)
                    .if_not_exists()
                    .col(pk_auto(Categories::Id))
                    .col(ColumnDef::new(Categories::Name).string().not_null())
                    .col(ColumnDef::new(Categories::Description).text())
                    .col(ColumnDef::new(Categories::ParentId).integer())
                    .col(
                        ColumnDef::new(Categories::CreatedAt)
                            .timestamp_with_time_zone()
                            .default("CURRENT_TIMESTAMP"),
                    )
                    .col(
                        ColumnDef::new(Categories::UpdatedAt)
                            .timestamp_with_time_zone()
                            .default("CURRENT_TIMESTAMP"),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(ProductCategories::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(ProductCategories::ProductId).integer().not_null())
                    .col(ColumnDef::new(ProductCategories::CategoryId).integer().not_null())
                    .primary_key(
                        Index::create()
                            .col(ProductCategories::ProductId)
                            .col(ProductCategories::CategoryId)
                    )
                    .to_owned(),
            )
            .await?;
        
        manager
            .create_table(
                Table::create()
                    .table(Reviews::Table)
                    .if_not_exists()
                    .col(pk_auto(Reviews::Id))
                    .col(ColumnDef::new(Reviews::ProductId).integer().not_null())
                    .col(ColumnDef::new(Reviews::UserId).integer().not_null())
                    .col(ColumnDef::new(Reviews::Rating).integer().not_null())
                    .col(ColumnDef::new(Reviews::Comment).text())
                    .col(
                        ColumnDef::new(Reviews::CreatedAt)
                            .timestamp_with_time_zone()
                            .default("CURRENT_TIMESTAMP"),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Refunds::Table)
                    .if_not_exists()
                    .col(pk_auto(Refunds::Id))
                    .col(ColumnDef::new(Refunds::PaymentId).integer().not_null())
                    .col(ColumnDef::new(Refunds::RefundAmount).decimal().not_null())
                    .col(ColumnDef::new(Refunds::RefundStatus).enumeration(Refunds::RefundStatus, vec![
                        RefundsStatus::Pending,
                        RefundsStatus::Processed,
                        RefundsStatus::Failed,
                        RefundsStatus::Completed,
                    ]))
                    .col(ColumnDef::new(Refunds::RefundReason).string())
                    .col(ColumnDef::new(Refunds::RefundRequestedAt).timestamp_with_time_zone().default("CURRENT_TIMESTAMP"))
                    .col(ColumnDef::new(Refunds::RefundProcessedAt).timestamp_with_time_zone())
                    .to_owned(),
            )
            .await?;

        

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Porducts::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Orders::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(OrderItems::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Payments::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(CartItems::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(ShippingInfo::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Coupons::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Categories::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(ProductCategories::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Reviews::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Refunds::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
    Name,
    Sex,
    Password,
    Birthday,
    Phone,
    Email,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Porducts {
    Table,
    Id,
    Name,
    Description,
    StockQuantity,
    Price,
    ImageUrl,
    CreatedAt,
    UpdatedAt,
}

// CREATE TABLE orders (
//     order_id INT PRIMARY KEY AUTO_INCREMENT,  -- 订单ID
//     user_id INT NOT NULL,  -- 用户ID（外键，指向users表）
//     total_price DECIMAL(10, 2) NOT NULL,  -- 订单总金额
//     status ENUM('pending', 'paid', 'shipped', 'completed', 'canceled', 'refunded') DEFAULT 'pending',  -- 订单状态
//     shipping_status ENUM('pending', 'shipped', 'in_transit', 'delivered', 'failed') DEFAULT 'pending',  -- 配送状态
//     shipping_company VARCHAR(100),  -- 配送公司（可选，配送信息来自shipping_info表）
//     tracking_number VARCHAR(100),  -- 物流单号（可选，配送信息来自shipping_info表）
//     payment_status ENUM('pending', 'paid', 'failed', 'refunded') DEFAULT 'pending',  -- 支付状态
//     payment_method ENUM('wechat', 'alipay', 'credit_card', 'paypal', 'bank_transfer') NOT NULL,  -- 支付方式
//     created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,  -- 订单创建时间
//     updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,  -- 订单更新时间
//     paid_at TIMESTAMP,  -- 支付时间
//     shipped_at TIMESTAMP,  -- 发货时间
//     delivered_at TIMESTAMP,  -- 配送完成时间
//     canceled_at TIMESTAMP,  -- 取消时间
//     refunded_at TIMESTAMP,  -- 退款时间
//     shipping_address TEXT NOT NULL,  -- 配送地址（如果支持物理配送）
//     billing_address TEXT NOT NULL,  -- 账单地址（用于发票等）
//     discount DECIMAL(10, 2) DEFAULT 0.00,  -- 订单折扣金额（如有）
//     coupon_code VARCHAR(50),  -- 优惠券代码（可选）
//     gift_card_code VARCHAR(50),  -- 礼品卡代码（可选）
//     notes TEXT,  -- 用户备注（如配送要求等）
//     FOREIGN KEY (user_id) REFERENCES users(user_id)  -- 外键关联到用户表
// );

#[derive(DeriveIden)]
enum Orders {
    Table,
    Id,
    UserId,
    TotalPrice,
    Status,
    ShippingStatus,
    ShippingCompany,
    TrackingNumber,
    PaymentStatus,
    PaymentMethod,
    CreatedAt,
    UpdatedAt,
    PaidAt,
    ShippedAt,
    DeliveredAt,
    CanceledAt,
    RefundedAt,
    ShippingAddress,
    BillingAddress,
    Discount,
    CouponCode,
    GiftCardCode,
    Notes,
}

#[derive(DeriveIden)]
enum ShippingStatus {
    Pending,
    Shipped,
    Delivered,
    Cancelled,
}

#[derive(DeriveIden)]
enum PaymentStatus{
    Pending,
    Paid,
    Failed,
    Refunded,
}

#[derive(DeriveIden)]
enum PaymentMethod {
    Wechat,
    Alipay,
    CreditCard,
    Paypal,
    BankTransfer,
    
}

#[derive(DeriveIden)]
enum OrdersStatus{
    Pending,
    Paid,
    Shipped,
    Completed,
    Canceled,
    Refunded,
}

// CREATE TABLE order_items (
//     order_item_id INT PRIMARY KEY AUTO_INCREMENT,  -- 订单商品ID
//     order_id INT NOT NULL,  -- 订单ID
//     product_id INT NOT NULL,  -- 商品ID
//     quantity INT NOT NULL,  -- 商品数量
//     price DECIMAL(10, 2) NOT NULL,  -- 商品价格
//     FOREIGN KEY (order_id) REFERENCES orders(order_id),
//     FOREIGN KEY (product_id) REFERENCES products(product_id)
// );
#[derive(DeriveIden)]
enum OrderItems {
    Table,
    Id,
    OrderId,
    ProductId,
    Quantity,
    Price,
}

// CREATE TABLE payments (
//     payment_id INT PRIMARY KEY AUTO_INCREMENT,  -- 支付ID
//     order_id INT NOT NULL,  -- 订单ID
//     payment_method ENUM('wechat', 'alipay') NOT NULL,  -- 支付方式（微信支付或支付宝支付）
//     transaction_id VARCHAR(100) NOT NULL,  -- 支付交易号（微信支付或支付宝支付）
//     pay_status ENUM('success', 'failed', 'pending') DEFAULT 'pending',  -- 支付状态
//     amount DECIMAL(10, 2) NOT NULL,  -- 支付金额
//     paid_at TIMESTAMP,  -- 支付时间
//     created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,  -- 创建时间
//     updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,  -- 更新时间
//     FOREIGN KEY (order_id) REFERENCES orders(order_id)
// );

#[derive(DeriveIden)]
enum Payments {
    Table,
    Id,
    OrderId,
    PaymentMethod,
    TransactionId,
    PayStatus,
    Amount,
    PaidAt,
    CreatedAt,
    UpdatedAt,
}

// CREATE TABLE cart_items (
//     cart_item_id INT PRIMARY KEY AUTO_INCREMENT,  -- 购物车商品ID
//     user_id INT NOT NULL,  -- 用户ID
//     product_id INT NOT NULL,  -- 商品ID
//     quantity INT NOT NULL,  -- 商品数量
//     added_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,  -- 加入购物车时间
//     FOREIGN KEY (user_id) REFERENCES users(user_id),
//     FOREIGN KEY (product_id) REFERENCES products(product_id)
// );
#[derive(DeriveIden)]
enum CartItems {
    Table,
    Id,
    UserId,
    ProductId,
    Quantity,
    AddedAt,
}

// CREATE TABLE shipping_info (
//     shipping_id INT PRIMARY KEY AUTO_INCREMENT,  -- 物流信息ID
//     order_id INT NOT NULL,  -- 订单ID
//     shipping_company VARCHAR(100) NOT NULL,  -- 快递公司名称
//     tracking_number VARCHAR(100) NOT NULL,  -- 物流单号
//     shipping_status ENUM('pending', 'shipped', 'in_transit', 'delivered', 'failed') DEFAULT 'pending',  -- 配送状态
//     estimated_delivery_date DATE,  -- 预计送达时间
//     shipped_at TIMESTAMP,  -- 发货时间
//     delivered_at TIMESTAMP,  -- 完成配送时间
//     created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,  -- 创建时间
//     updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,  -- 更新时间
//     FOREIGN KEY (order_id) REFERENCES orders(order_id)
// );
#[derive(DeriveIden)]
enum ShippingInfo {
    Table,
    Id,
    OrderId,
    ShippingCompany,
    TrackingNumber,
    ShippingStatus,
    EstimatedDeliveryDate,
    ShippedAt,
    DeliveredAt,
    CreatedAt,
    UpdatedAt,
}

// CREATE TABLE coupons (
//     coupon_id INT PRIMARY KEY AUTO_INCREMENT,  -- 优惠券ID
//     code VARCHAR(50) NOT NULL,  -- 优惠券代码
//     discount DECIMAL(10, 2) NOT NULL,  -- 优惠金额
//     valid_from TIMESTAMP,  -- 开始时间
//     valid_until TIMESTAMP,  -- 结束时间
//     usage_count INT DEFAULT 0,  -- 已使用次数
//     total_count INT DEFAULT 0,  -- 总发行次数
//     created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP  -- 创建时间
// );
#[derive(DeriveIden)]
enum Coupons {
    Table,
    Id,
    Code,
    Discount,
    ValidFrom,
    ValidUntil,
    UsageCount,
    TotalCount,
    CreatedAt,
}

// CREATE TABLE categories (
//     category_id INT PRIMARY KEY AUTO_INCREMENT,  -- 分类ID
//     name VARCHAR(100) NOT NULL,  -- 分类名称
//     description TEXT,  -- 分类描述
//     parent_id INT,  -- 父级分类ID（支持多级分类）
//     created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,  -- 创建时间
//     updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,  -- 更新时间
//     FOREIGN KEY (parent_id) REFERENCES categories(category_id)
// );
#[derive(DeriveIden)]
enum Categories {
    Table,
    Id,
    Name,
    Description,
    ParentId,
    CreatedAt,
    UpdatedAt,
}

// CREATE TABLE product_categories (
//     product_id INT NOT NULL,  -- 商品ID
//     category_id INT NOT NULL,  -- 分类ID
//     PRIMARY KEY (product_id, category_id),
//     FOREIGN KEY (product_id) REFERENCES products(product_id),
//     FOREIGN KEY (category_id) REFERENCES categories(category_id)
// );
#[derive(DeriveIden)]
enum ProductCategories {
    Table,
    ProductId,
    CategoryId,
}

// CREATE TABLE reviews (
//     review_id INT PRIMARY KEY AUTO_INCREMENT,  -- 评论ID
//     product_id INT NOT NULL,  -- 商品ID
//     user_id INT NOT NULL,  -- 用户ID
//     rating INT NOT NULL,  -- 评分（1-5分）
//     comment TEXT,  -- 评论内容
//     created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,  -- 评论时间
//     FOREIGN KEY (product_id) REFERENCES products(product_id),
//     FOREIGN KEY (user_id) REFERENCES users(user_id)
// );
#[derive(DeriveIden)]
enum Reviews {
    Table,
    Id,
    ProductId,
    UserId,
    Rating,
    Comment,
    CreatedAt,
}

// CREATE TABLE refunds (
//     refund_id INT PRIMARY KEY AUTO_INCREMENT,  -- 退款ID
//     payment_id INT NOT NULL,  -- 支付ID
//     refund_amount DECIMAL(10, 2) NOT NULL,  -- 退款金额
//     refund_status ENUM('pending', 'processed', 'failed', 'completed') DEFAULT 'pending',  -- 退款状态
//     refund_reason VARCHAR(255),  -- 退款原因
//     refund_requested_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,  -- 退款申请时间
//     refund_processed_at TIMESTAMP,  -- 退款处理时间
//     FOREIGN KEY (payment_id) REFERENCES payments(payment_id)
// );

#[derive(DeriveIden)]
enum Refunds {
    Table,
    Id,
    PaymentId,
    RefundAmount,
    RefundStatus,
    RefundReason,
    RefundRequestedAt,
    RefundProcessedAt,
}

#[derive(DeriveIden)]
enum RefundsStatus {
    Pending,
    Processed,
    Failed,
    Completed,
    
}
