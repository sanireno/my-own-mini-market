-- Add migration script here
CREATE TABLE users (
                       id BIGSERIAL PRIMARY KEY,
                       username TEXT NOT NULL UNIQUE,
                       email TEXT NOT NULL UNIQUE,
                       password_hash TEXT NOT NULL,
                       created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);