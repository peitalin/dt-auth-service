-- Your SQL goes here
ALTER TABLE users
ADD COLUMN last_seen TIMESTAMP DEFAULT current_timestamp;