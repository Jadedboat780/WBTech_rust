use tokio_postgres::Client;
use crate::models::{CreateOrder, GetOrder};

type PgResult<T> = Result<T, tokio_postgres::Error>;

pub async fn select_order_by_id(track_number: String, client: &Client) -> PgResult<GetOrder> {
    let query = "SELECT o.order_uid,
                           o.track_number,
                           o.entry,
                           json_build_object(
                                   'name', D.name,
                                   'phone', D.phone,
                                   'zip', D.zip,
                                   'city', D.city,
                                   'address', D.address,
                                   'region', D.region,
                                   'email', D.email
                           ) AS delivery,
                           json_build_object(
                                   'transaction', P.transaction,
                                   'request_id', P.request_id,
                                   'currency', P.currency,
                                   'provider', P.provider,
                                   'amount', P.amount,
                                   'payment_dt', P.payment_dt,
                                   'bank', P.bank,
                                   'delivery_cost', P.delivery_cost,
                                   'goods_total', P.goods_total,
                                   'custom_fee', P.custom_fee
                           ) AS payment,
                           json_agg(
                                   json_build_object(
                                           'chrt_id', I.chrt_id,
                                           'track_number', I.track_number,
                                           'price', I.price,
                                           'rid', I.rid,
                                           'name', I.name,
                                           'sale', I.sale,
                                           'size', I.size,
                                           'total_price', I.total_price,
                                           'nm_id', I.nm_id,
                                           'brand', I.brand,
                                           'status', I.status
                                   ) ORDER BY I.chrt_id
                           ) AS items,
                           o.locale,
                           o.internal_signature,
                           o.customer_id,
                           o.delivery_service,
                           o.shardkey,
                           o.sm_id,
                           o.date_created,
                           o.oof_shard
                    FROM Orders o
                             JOIN Deliveries AS D ON o.delivery_id = D.id
                             JOIN Payments AS P ON o.payment_id = P.id
                             JOIN OrderItems AS OI ON o.order_uid = OI.order_uid
                             JOIN Items I ON OI.item_id = I.id
                    WHERE o.track_number = $1
                    GROUP BY o.order_uid, D.id, P.id;";

    let row = client.query_one(query, &[&track_number]).await?;

    let order = GetOrder::from(row);
    Ok(order)
}

pub async fn insert_order(order: CreateOrder, client: &Client) -> PgResult<u32> {
    // Вставка данных в таблицу Deliveries
    let delivery_id: i32 = client.query_one(
        r#"
        INSERT INTO Deliveries (name, phone, zip, city, address, region, email)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING id;
        "#,
        &[
            &order.delivery["name"].to_string(),
            &order.delivery["phone"].to_string(),
            &order.delivery["zip"].to_string(),
            &order.delivery["city"].to_string(),
            &order.delivery["address"].to_string(),
            &order.delivery["region"].to_string(),
            &order.delivery["email"].to_string(),
        ],
    ).await?.get(0);

    // Вставка данных в таблицу Payments
    let payment_id: i32 = client.query_one(
        r#"
        INSERT INTO Payments (transaction, request_id, currency, provider, amount, payment_dt, bank, delivery_cost, goods_total, custom_fee)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
        RETURNING id;
        "#,
        &[
            &order.payment["transaction"].to_string(),
            &order.payment["request_id"].to_string(),
            &order.payment["currency"].to_string(),
            &order.payment["provider"].to_string(),
            &(order.payment["amount"].as_i64().unwrap() as i32),
            &(order.payment["payment_dt"].as_i64().unwrap() as i32),
            &order.payment["bank"].to_string(),
            &(order.payment["delivery_cost"].as_i64().unwrap() as i32),
            &(order.payment["goods_total"].as_i64().unwrap() as i32),
            &(order.payment["custom_fee"].as_i64().unwrap() as i32)
        ],
    ).await?.get(0);

    // Вставка данных в таблицу Orders
    client.execute(
        r#"
        INSERT INTO Orders (
            order_uid, track_number, entry, delivery_id, payment_id, locale,
            internal_signature, customer_id, delivery_service, shardkey, sm_id, date_created, oof_shard
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13);
        "#,
        &[
            &uuid::Uuid::new_v4(),
            &order.track_number,
            &order.entry,
            &delivery_id,
            &payment_id,
            &order.locale,
            &order.internal_signature,
            &order.customer_id,
            &order.delivery_service,
            &order.shardkey,
            &order.sm_id,
            &order.date_created,
            &order.oof_shard,
        ],
    ).await?;

    // Вставка данных в таблицу OrderItems
    for item in order.items.as_array().expect("Items should be an array") {
        // Вставляем данные в таблицу Items
        let item_id: i32 = client.query_one(
            r#"
            INSERT INTO Items (chrt_id, track_number, price, rid, name, sale, size, total_price, nm_id, brand, status)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            RETURNING id;
            "#,
            &[
                &(item["chrt_id"].as_i64().unwrap() as i32),
                &item["track_number"].to_string(),
                &(item["price"].as_i64().unwrap() as i32),
                &item["rid"].to_string(),
                &item["name"].to_string(),
                &(item["sale"].as_i64().unwrap() as i32),
                &item["size"].to_string(),
                &(item["total_price"].as_i64().unwrap() as i32),
                &(item["nm_id"].as_i64().unwrap() as i32),
                &item["brand"].to_string(),
                &(item["status"].as_i64().unwrap() as i32),
            ],
        ).await?.get(0);

        // Теперь связываем Order и Items в таблице OrderItems
        // client.execute(
        //     r#"
        //     INSERT INTO OrderItems (order_uid, item_id)
        //     VALUES ($1, $2);
        //     "#,
        //     &[
        //         &uuid::Uuid::new_v4(),
        //         &item_id,
        //     ],
        // ).await?;
    }

    Ok(32)
}

