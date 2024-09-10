use crate::models::{CreateDelivery, CreateItem, CreateOrder, CreatePayment, GetOrder};
use tokio_postgres::Client;

/// Результат запроса в базу данных
type PgResult<T> = Result<T, tokio_postgres::Error>;

/// Получение данных о заказе из базы данных
pub async fn select_order_by_id(track_number: String, client: &Client) -> PgResult<GetOrder> {
    let query = "
    SELECT O.order_uid,
    O.track_number,
    O.entry,
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
       O.locale, O.internal_signature, O.customer_id, O.delivery_service, O.shardkey, O.sm_id, O.date_created, O.oof_shard
       FROM Orders AS O
                 JOIN Deliveries AS D ON O.delivery_id = D.id
                 JOIN Payments AS P ON O.payment_id = P.id
                 JOIN Items AS I ON I.track_number = O.track_number
        WHERE O.track_number = $1
        GROUP BY O.order_uid, D.id, P.id;
";

    let row = client.query_one(query, &[&track_number]).await?;

    let order = GetOrder::from(row);
    Ok(order)
}

/// Добавление данных о доставке
async fn insert_delivery(delivery: CreateDelivery, client: &Client) -> PgResult<i32> {
    let query = "
    INSERT INTO Deliveries (name, phone, zip, city, address, region, email)
    VALUES ($1, $2, $3, $4, $5, $6, $7)
    RETURNING id;
    ";

    let delivery_id = client.query_one(
        query,
        &[&delivery.name,
            &delivery.phone,
            &delivery.zip,
            &delivery.city,
            &delivery.address,
            &delivery.region,
            &delivery.email],
    )
        .await?
        .get(0);

    Ok(delivery_id)
}

/// Добавление данных о платеже
async fn insert_payment(payment: CreatePayment, client: &Client) -> PgResult<i32> {
    let query = "
    INSERT INTO Payments (transaction, request_id, currency, provider, amount, payment_dt, bank, delivery_cost, goods_total, custom_fee)
    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
    RETURNING id;
    ";

    let payment_id = client.query_one(
        query,
        &[
            &payment.transaction,
            &payment.request_id,
            &payment.currency,
            &payment.provider,
            &payment.amount,
            &payment.payment_dt,
            &payment.bank,
            &payment.delivery_cost,
            &payment.goods_total,
            &payment.custom_fee
        ],
    )
        .await?
        .get(0);

    Ok(payment_id)
}

/// Добавление данных о товаре
async fn insert_item(item: CreateItem, client: &Client) -> PgResult<()> {
    let query = "
    INSERT INTO Items (chrt_id, track_number, price, rid, name, sale, size, total_price, nm_id, brand, status)
    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
    RETURNING chrt_id;
    ";

    let _ = client.query_one(
        query,
        &[
            &item.chrt_id,
            &item.track_number,
            &item.price,
            &item.rid,
            &item.name,
            &item.sale,
            &item.size,
            &item.total_price,
            &item.nm_id,
            &item.brand,
            &item.status
        ],
    ).await?;

    Ok(())
}

/// Добавление данных о заказе
pub async fn insert_order(order: CreateOrder, client: &Client) -> PgResult<()> {
    let delivery_id = insert_delivery(order.delivery, client).await?;
    let payment_id = insert_payment(order.payment, client).await?;

    for item in order.items {
        let _item_id = insert_item(item, client).await?;
    }

    let query = "
    INSERT INTO Orders (
    track_number, entry, delivery_id, payment_id, locale,
    internal_signature, customer_id, delivery_service, shardkey, sm_id, date_created, oof_shard
    )
    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12);
    ";

    client.execute(
        query,
        &[
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
    )
        .await?;

    Ok(())
}
