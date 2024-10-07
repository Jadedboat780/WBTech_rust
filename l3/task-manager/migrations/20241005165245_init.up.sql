CREATE TABLE Tasks
(
    id        SERIAL PRIMARY KEY,
    title     TEXT NOT NULL,
    completed BOOLEAN DEFAULT FALSE
);

