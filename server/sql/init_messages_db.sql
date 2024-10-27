-- Add down migration script here

CREATE TABLE
    IF NOT EXISTS users (
        id BIGINT PRIMARY KEY UNIQUE NOT NULL,
        username VARCHAR(48) NOT NULL,
        password_hash VARCHAR(256) NOT NULL,
        salt VARCHAR(64) NOT NULL,
        email VARCHAR(64),
        created_at BIGINT NOT NULL,
        valid_refresh_token VARCHAR(1024),
        verified BOOLEAN NOT NULL
    );