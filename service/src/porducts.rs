use ::entity::{product_categories::{self, Entity as ProductCategory, Column as ProductCategoryColumn}, products::{self, Entity as Product}, categories};
use chrono::{DateTime, Utc};
use prelude::DateTimeWithTimeZone;
use sea_orm::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct PorductModel {
    pub name: String,
    pub status: i32,
    pub description: Option<String>,
    pub stock_quantity: i32,
    pub price: prelude::Decimal,
    pub image_url: Option<String>,
    pub sku: Option<String>,
    pub type_name: Option<String>,
    pub brand: Option<String>,
    pub product_details: Option<String>,
    pub product_information: Option<String>,
    pub configuration_list: Option<String>,
    pub wass: Option<String>,
    pub is_new: i32,
}

pub struct PorductServices;

impl PorductServices {
    pub async fn create_porduct(
        db: &DbConn,
        form_data: PorductModel,
    ) -> Result<products::ActiveModel, DbErr> {
        products::ActiveModel {
            name: Set(form_data.name.to_owned()),
            status: Set(form_data.status),
            description: Set(form_data.description.to_owned()),
            stock_quantity: Set(form_data.stock_quantity),
            price: Set(form_data.price),
            image_url: Set(form_data.image_url.to_owned()),
            sku: Set(form_data.sku.to_owned()),
            type_name: Set(form_data.type_name.to_owned()),
            brand: Set(form_data.brand.to_owned()),
            product_details: Set(form_data.product_details.to_owned()),
            product_information: Set(form_data.product_information.to_owned()),
            configuration_list: Set(form_data.configuration_list.to_owned()),
            wass: Set(form_data.wass.to_owned()),
            is_new: Set(form_data.is_new),
            created_at: Set(DateTimeWithTimeZone::from(Utc::now())),
            updated_at: Set(DateTimeWithTimeZone::from(Utc::now())),
            ..Default::default()
        }
        .save(db)
        .await
    }

    pub async fn update_porduct_by_id(
        db: &DbConn,
        id: i32,
        form_data: PorductModel,
    ) -> Result<products::Model, DbErr> {
        let products: products::ActiveModel = Product::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find products.".to_owned()))
            .map(Into::into)?;
        products::ActiveModel {
            id: products.id,
            name: Set(form_data.name.to_owned()),
            status: Set(form_data.status),
            description: Set(form_data.description.to_owned()),
            stock_quantity: Set(form_data.stock_quantity),
            price: Set(form_data.price),
            image_url: Set(form_data.image_url.to_owned()),
            sku: Set(form_data.sku.to_owned()),
            type_name: Set(form_data.type_name.to_owned()),
            brand: Set(form_data.brand.to_owned()),
            product_details: Set(form_data.product_details.to_owned()),
            product_information: Set(form_data.product_information.to_owned()),
            configuration_list: Set(form_data.configuration_list.to_owned()),
            is_new: Set(form_data.is_new),
            updated_at: Set(DateTimeWithTimeZone::from(Utc::now())),
            ..Default::default()
        }
        .update(db)
        .await
    }

    pub async fn delete_porduct_by_id(db: &DbConn, id: i32) -> Result<DeleteResult, DbErr> {
        let products: products::ActiveModel = Product::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find products.".to_owned()))
            .map(Into::into)?;
        products.delete(db).await
    }

    pub async fn get_porducts(db: &DbConn) -> Result<Vec<products::Model>, DbErr> {
        Product::find().all(db).await
    }

    // fenye
    pub async fn get_porducts_by_page(
        db: &DbConn,
        page: u64,
        size: u64,
        q: Option<String>,
        categories_id: Option<i32>,
    ) -> Result<(Vec<products::Model>, u64, u64), DbErr> {

        let mut query = Product::find();

        // 如果提供了分类ID，则添加分类过滤
        if let Some(cat_id) = categories_id {
            query = query
                .join(JoinType::LeftJoin, products::Relation::ProductCategories.def())
                .filter(product_categories::Column::CategoryId.eq(cat_id));
        }

        // 如果提供了查询条件，则添加过滤条件
        if let Some(query_string) = q {
            query = query.filter(products::Column::Name.like(format!("%{}%", query_string).as_str()));
        }

        // 按 ID 升序排序
        let paginator = query.order_by_asc(products::Column::Id).paginate(db, size);

        // 获取总记录数
        let total_items = paginator.num_items().await?;

        // 计算总页数
        let total_pages = (total_items + size - 1) / size;

        // 获取指定页的数据
        let products = paginator.fetch_page(page.saturating_sub(1)).await?;

        Ok((products, total_pages, total_items))
    }

    pub async fn get_porduct_by_id(db: &DbConn, id: i32) -> Result<products::Model, DbErr> {
        Product::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find porduct.".to_owned()))
    }

    pub async fn get_product_by_new(db: &DbConn) -> Result<Vec<products::Model>, DbErr> {
        Product::find()
            .filter(products::Column::IsNew.eq(1))
            .order_by_desc(products::Column::CreatedAt)
            .limit(5)
            .all(db)
            .await
    }
}
