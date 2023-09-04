-- Your SQL goes here
CREATE TABLE users(
    id BIGSERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    email TEXT NOT NULL UNIQUE,
    salt TEXT NOT NULL,
    salted_password TEXT NOT NULL
)