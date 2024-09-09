-- Таблица для хранения информации о доставке
CREATE TABLE IF NOT EXISTS Deliveries
(
    id      SERIAL PRIMARY KEY,
    name    VARCHAR      NOT NULL,
    phone   VARCHAR(12)  NOT NULL,
    zip     VARCHAR(10)  NOT NULL,
    city    VARCHAR(100) NOT NULL,
    address VARCHAR      NOT NULL,
    region  VARCHAR(100) NOT NULL,
    email   VARCHAR      NOT NULL
);

-- Таблица для хранения информации о платежах
CREATE TABLE IF NOT EXISTS Payments
(
    id            SERIAL PRIMARY KEY,
    transaction   VARCHAR      NOT NULL,
    request_id    VARCHAR      NOT NULL,
    currency      VARCHAR(10)  NOT NULL,
    provider      VARCHAR(100) NOT NULL,
    amount        INT          NOT NULL,
    payment_dt    INT          NOT NULL,
    bank          VARCHAR(100) NOT NULL,
    delivery_cost INT          NOT NULL,
    goods_total   INT          NOT NULL,
    custom_fee    INT          NOT NULL
);

-- Таблица для хранения информации о товарах
CREATE TABLE IF NOT EXISTS Items
(
    id           SERIAL PRIMARY KEY,
    chrt_id      INT         NOT NULL,
    track_number VARCHAR     NOT NULL,
    price        INT         NOT NULL,
    rid          VARCHAR     NOT NULL,
    name         VARCHAR     NOT NULL,
    sale         INT         NOT NULL,
    size         VARCHAR(50) NOT NULL,
    total_price  INT         NOT NULL,
    nm_id        INT         NOT NULL,
    brand        VARCHAR     NOT NULL,
    status       INT         NOT NULL
);

-- Таблица для хранения заказов
CREATE TABLE IF NOT EXISTS Orders
(
    order_uid          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    track_number       VARCHAR   UNIQUE NOT NULL,
    entry              VARCHAR   NOT NULL,
    delivery_id        SERIAL REFERENCES Deliveries (id) ON DELETE CASCADE,
    payment_id         SERIAL REFERENCES Payments (id) ON DELETE CASCADE,
    locale             VARCHAR    NOT NULL,
    internal_signature VARCHAR   NOT NULL,
    customer_id        VARCHAR   NOT NULL,
    delivery_service   VARCHAR   NOT NULL,
    shardkey           VARCHAR   NOT NULL,
    sm_id              SERIAL    NOT NULL,
    date_created       TIMESTAMP NOT NULL,
    oof_shard          VARCHAR   NOT NULL
);

-- Таблица для хранения связи заказов и товаров
CREATE TABLE OrderItems
(
    order_uid UUID REFERENCES Orders (order_uid) ON DELETE CASCADE,
    item_id   INT REFERENCES Items (id) ON DELETE CASCADE,
    PRIMARY KEY (order_uid, item_id)
);

-- Добавление тестовых данных
INSERT INTO Deliveries (name, phone, zip, city, address, region, email)
VALUES ('Test Testov', '+9720000000', '2639809', 'Kiryat Mozkin', 'Ploshad Mira 15', 'Kraiot', 'test@gmail.com');

INSERT INTO Payments (transaction, request_id, currency, provider, amount, payment_dt, bank, delivery_cost, goods_total, custom_fee)
VALUES ('b563feb7b2b84b6test', '', 'USD', 'wbpay', 1817, 1637907727, 'alpha', 1500, 317, 0);

INSERT INTO Items (chrt_id, track_number, price, rid, name, sale, size, total_price, nm_id, brand, status)
VALUES (9934930, 'WBILMTESTTRACK', 453, 'ab4219087a764ae0btest', 'Mascaras', 30, '0', 317, 2389212, 'Vivienne Sabo', 202),
       (9934943, 'WBILMTESTTRACK12', 600, 'ab4219087a764ae0bte234', 'Mascardfs', 20, '1', 320, 2383212, 'Vivienne',205);

INSERT INTO Orders (track_number, entry, delivery_id, payment_id, locale, internal_signature, customer_id, delivery_service,
                    shardkey, sm_id, date_created,oof_shard)
VALUES ('WBILMTESTTRACK','WBIL',
        (SELECT id FROM deliveries ORDER BY id DESC LIMIT 1),
        (SELECT id FROM payments ORDER BY id DESC LIMIT 1),
        'EN','','test','meest','9',99,'2021-11-26 06:22:19','1');

INSERT INTO OrderItems (order_uid, item_id)
VALUES ((SELECT order_uid FROM Orders LIMIT 1),
        (SELECT id FROM Items LIMIT 1));

INSERT INTO OrderItems (order_uid, item_id)
VALUES ((SELECT order_uid FROM Orders LIMIT 1),
        (SELECT id FROM Items ORDER BY id DESC LIMIT 1));
