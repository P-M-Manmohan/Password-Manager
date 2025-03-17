-- Add migration script here
CREATE TABLE IF NOT EXISTS passwords (
    id SERIAL PRIMARY KEY,
    service TEXT UNIQUE NOT NULL,
    username TEXT NOT NULL,
    password TEXT NOT NULL,
    nonce_user TEXT NOT NULL,
    nonce_pass TEXT NOT NULL
);

