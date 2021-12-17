-- Add migration script here
CREATE TABLE IF NOT EXISTS channels
(
    name VARCHAR PRIMARY KEY NOT NULL,
    id BIGINT NOT NULL
);