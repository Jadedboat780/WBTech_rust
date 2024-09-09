use serde::{Deserialize, Serialize};
use serde_json::Value;
use chrono::NaiveDateTime;

#[derive(Serialize, Debug)]
pub struct GetOrder {
    order_uid: uuid::Uuid,
    track_number: String,
    entry: String,
    delivery: Value,
    payment:Value,
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

#[derive(Deserialize, Debug)]
pub struct CreateOrder {
    pub track_number: String,
    pub entry: String,
    pub delivery: Value,
    pub payment:Value,
    pub items: Value,
    pub locale: String,
    pub internal_signature: String,
    pub customer_id: String,
    pub delivery_service: String,
    pub shardkey: String,
    pub sm_id: i32,
    pub date_created: NaiveDateTime,
    pub oof_shard: String,
}

impl From<tokio_postgres::Row> for GetOrder {
    fn from(row: tokio_postgres::Row) -> Self {
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

// #[derive(Serialize, Deserialize, Debug)]
// pub struct Delivery {
//     id: u32,
//     name: String,
//     phone: String,
//     zip: String,
//     city: String,
//     address: String,
//     region: String,
//     email: String,
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

// impl From<tokio_postgres::Row> for Delivery {
//     fn from(row: tokio_postgres::Row) -> Self {
//         Delivery {
//             id: row.get("id"),
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