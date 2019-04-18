-- Your SQL goes here

CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  login TEXT NOT NULL,
  password TEXT NOT NULL
);

INSERT INTO "users" (id, login, password) VALUES
(0, 'admin', 'admin');