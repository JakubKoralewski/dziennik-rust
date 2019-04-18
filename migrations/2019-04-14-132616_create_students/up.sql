-- Your SQL goes here
CREATE TABLE students (
  id SERIAL PRIMARY KEY,
  first_name TEXT NOT NULL,
  last_name TEXT NOT NULL,
  class TEXT NOT NULL,
  phone_number INTEGER NOT NULL
);

INSERT INTO "students" (id, first_name, last_name, class, phone_number) VALUES
(0, 'Jakub', 'Koralewski', '3d', 66666666);