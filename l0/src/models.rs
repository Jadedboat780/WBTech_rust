use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio_postgres::Row;

/// Модель для получения данных о заказе
#[derive(Serialize, Debug)]
pub struct GetOrder {
    order_uid: uuid::Uuid,
    track_number: String,
    entry: String,
    delivery: Value,
    payment: Value,
    items: Value,
    locale: String,
    internal_signature: String,
    customer_id: String,
    delivery_service: String,
    shardkey: String,
    sm_id: i32,
    date_created: NaiveDateTime,
    oof_shard: String,
}

/// Модель для создания заказа
#[derive(Deserialize, Debug)]
pub struct CreateOrder {
    pub track_number: String,
    pub entry: String,
    pub delivery: CreateDelivery,
    pub payment: CreatePayment,
    pub items: Vec<CreateItem>,
    pub locale: String,
    pub internal_signature: String,
    pub customer_id: String,
    pub delivery_service: String,
    pub shardkey: String,
    pub sm_id: i32,
    pub date_created: NaiveDateTime,
    pub oof_shard: String,
}

/// Модель для создания информации о доставки
#[derive(Deserialize, Debug)]
pub struct CreateDelivery {
    pub name: String,
    pub phone: String,
    pub zip: String,
    pub city: String,
    pub address: String,
    pub region: String,
    pub email: String,
}

/// Модель для создания информации о платеже
#[derive(Deserialize, Debug)]
pub struct CreatePayment {
    pub transaction: String,
    pub request_id: String,
    pub currency: String,
    pub provider: String,
    pub amount: i32,
    pub payment_dt: i32,
    pub bank: String,
    pub delivery_cost: i32,
    pub goods_total: i32,
    pub custom_fee: i32,
}

/// Модель для создания информации о товаре
#[derive(Deserialize, Debug)]
pub struct CreateItem {
    pub chrt_id: i32,
    pub track_number: String,
    pub price: i32,
    pub rid: String,
    pub name: String,
    pub sale: i32,
    pub size: String,
    pub total_price: i32,
    pub nm_id: i32,
    pub brand: String,
    pub status: i32,
}

impl From<Row> for GetOrder {
    fn from(row: Row) -> Self {
        GetOrder {
            order_uid: row.get("order_uid"),
            track_number: row.get("track_number"),
            entry: row.get("entry"),
            delivery: row.get("delivery"),
            payment: row.get("payment"),
            items: row.get("items"),
            locale: row.get("locale"),
            internal_signature: row.get("internal_signature"),
            customer_id: row.get("customer_id"),
            delivery_service: row.get("delivery_service"),
            shardkey: row.get("shardkey"),
            sm_id: row.get("sm_id"),
            date_created: row.get("date_created"),
            oof_shard: row.get("oof_shard"),
        }
    }
}

// impl From<Row> for GetDelivery {
//     fn from(row: Row) -> Self {
//         GetDelivery {
//             name: row.get("name"),
//             phone: row.get("phone"),
//             zip: row.get("zip"),
//             city: row.get("city"),
//             address: row.get("address"),
//             region: row.get("region"),
//             email: row.get("email"),
//         }
//     }
// }

//
// #[derive(Serialize, Deserialize, Debug, FromSql)]
// pub struct Payment {
//     id: u32,
//     transaction: String,
//     request_id: String,
//     currency: String,
//     provider: String,
//     amount: u32,
//     payment_dt: u32,
//     bank: String,
//     delivery_cost: u32,
//     goods_total: u32,
//     custom_fee: u32,
// }
//
// #[derive(Serialize, Deserialize, Debug)]
// pub struct Item {
//     id: u32,
//     chrt_id: u32,
//     track_number: String,
//     price: u32,
//     rid: String,
//     name: String,
//     sale: u32,
//     size: String,
//     total_price: u32,
//     nm_id: u32,
//     brand: String,
//     status: u32,
// }

// impl From<&tokio_postgres::Row> for Payment {
//     fn from(row: &tokio_postgres::Row) -> Self {
//         Payment {
//             id: row.get("id"),
//             transaction: row.get("transaction"),
//             request_id: row.get("request_id"),
//             currency: row.get("currency"),
//             provider: row.get("provider"),
//             amount: row.get("amount"),
//             payment_dt: row.get("payment_dt"),
//             bank: row.get("bank"),
//             delivery_cost: row.get("delivery_cost"),
//             goods_total: row.get("goods_total"),
//             custom_fee: row.get("custom_fee"),
//         }
//     }
// }
//
// impl From<&tokio_postgres::Row> for Item {
//     fn from(row: &tokio_postgres::Row) -> Self {
//         Item {
//             id: row.get("id"),
//             chrt_id: row.get("chrt_id"),
//             track_number: row.get("track_number"),
//             price: row.get("price"),
//             rid: row.get("rid"),
//             name: row.get("name"),
//             sale: row.get("sale"),
//             size: row.get("size"),
//             total_price: row.get("total_price"),
//             nm_id: row.get("nm_id"),
//             brand: row.get("brand"),
//             status: row.get("status"),
//         }
//     }
// }
