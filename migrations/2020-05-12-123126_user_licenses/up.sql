-- Your SQL goes here
CREATE TABLE user_licenses (
    id TEXT PRIMARY KEY,
    license_number TEXT NOT NULL,
    license_category TEXT,
    expiry TIMESTAMP NOT NULL,
    state TEXT,
    verified BOOLEAN NOT NULL,
)
