-- Your SQL goes here

CREATE TABLE paths (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  description TEXT NOT NULL
)