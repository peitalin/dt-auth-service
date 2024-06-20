-- Your SQL goes here
ALTER TABLE following_stores
ADD COLUMN last_visited TIMESTAMP DEFAULT current_timestamp;