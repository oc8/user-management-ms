-- Your SQL goes here
CREATE TABLE users (
   id uuid DEFAULT gen_random_uuid(),
   email VARCHAR(255) UNIQUE NOT NULL,
   PRIMARY KEY (id)
);