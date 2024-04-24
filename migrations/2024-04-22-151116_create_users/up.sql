-- Your SQL goes here
CREATE TABLE users (
   id uuid DEFAULT gen_random_uuid(),
   email VARCHAR(255) UNIQUE NOT NULL,
   otp_secret VARCHAR(255) NOT NULL,
   PRIMARY KEY (id)
);