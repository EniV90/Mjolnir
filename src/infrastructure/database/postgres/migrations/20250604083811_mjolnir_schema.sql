-- Add migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE users (
    id  UUID PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
    username TEXT NOT NULL,
    password_hash TEXT NOT NULL,
    password_salt TEXT NOT NULL,
    email TEXT UNIQUE NOT NULL,
    active BOOLEAN NOT NULL DEFAULT TRUE,
    roles TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL
);

INSERT INTO users (
    username,
    email,
    password_hash,
    password_salt,
    active,
    roles,
    created_at,
    updated_at
)
VALUES (
    'admin',
    'admin@admin.com',
    '10fad756ef1f6c18903abe9f94cc83259050ede763241820982f0d5464d3d9',
    'mAa5lJGdETj',
    true,
    'admin',
    now(),
    now()
);