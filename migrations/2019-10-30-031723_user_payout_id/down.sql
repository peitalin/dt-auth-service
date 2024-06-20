-- This file should undo anything in `up.sql`

ALTER TABLE users
ADD COLUMN payout_method TEXT;

ALTER TABLE users
DROP COLUMN payout_method_id;