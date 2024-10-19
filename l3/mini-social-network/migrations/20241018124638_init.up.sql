CREATE TABLE IF NOT EXISTS Users
(
    id       UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name     VARCHAR,
    password VARCHAR
);

CREATE TABLE IF NOT EXISTS Posts
(
    id      UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES Users (id) ON DELETE CASCADE,
    content VARCHAR,
    likes   SERIAL
);