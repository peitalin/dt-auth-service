-- Your SQL goes here
CREATE TABLE users (
    id TEXT PRIMARY KEY,
    email TEXT NOT NULL UNIQUE,
    username TEXT,
    first_name TEXT,
    last_name TEXT,
    password_hash TEXT NOT NULL,
    email_verified BOOL NOT NULL,
    created_at TIMESTAMP DEFAULT current_timestamp,
    updated_at TIMESTAMP,
    cart_id TEXT,
    payment_method_ids TEXT[] NOT NULL,
    default_payment_method_id TEXT,
    is_suspended BOOLEAN NOT NULL,
    store_id TEXT,
    user_role TEXT,
    payout_method TEXT
);

CREATE OR REPLACE FUNCTION trigger_set_timestamp()
RETURNS TRIGGER AS $$
BEGIN
  NEW.updated_at = NOW();
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER set_timestamp
BEFORE UPDATE ON users
FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();