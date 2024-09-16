-- Add down migration script here

CREATE TABLE
    IF NOT EXISTS messages (
        id BIGINT PRIMARY KEY NOT NULL UNIQUE,
        content TEXT NOT NULL,
        author_id INT NOT NULL,
        channel_id INT NOT NULL,
        created_at BIGINT,
        updated_at BIGINT
    );  