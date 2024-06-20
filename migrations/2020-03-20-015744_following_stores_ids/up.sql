-- Your SQL goes here

ALTER TABLE users
ADD COLUMN following_stores_ids TEXT[] NOT NULL DEFAULT array[]::TEXT[];