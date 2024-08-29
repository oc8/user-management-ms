CREATE TABLE "user" (
    id uuid DEFAULT gen_random_uuid(),
    email VARCHAR(255) UNIQUE NOT NULL,
    otp_secret VARCHAR(255) NOT NULL,
    created_at TIMESTAMP DEFAULT now() NOT NULL,
    PRIMARY KEY (id)
);