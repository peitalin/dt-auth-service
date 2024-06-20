-- Your SQL goes here

ALTER TABLE users
ADD COLUMN payout_method_id TEXT;

ALTER TABLE users
DROP COLUMN payout_method;