-- Your SQL goes here
CREATE TABLE following_stores (
    PRIMARY KEY(user_id, store_id),
    user_id TEXT NOT NULL,
    store_id TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT current_timestamp
);

