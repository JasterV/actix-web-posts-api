-- Your SQL goes here
CREATE TABLE posts (
    id SERIAL,
    name VARCHAR(20) NOT NULL UNIQUE
);