CREATE TYPE provider_type AS ENUM (
    'local',
    'passwordless'
);

CREATE TABLE provider (
    id uuid DEFAULT gen_random_uuid(),
    user_id uuid NOT NULL,
    provider provider_type NOT NULL,
    password VARCHAR(255),
    token VARCHAR(255),
    refresh_token VARCHAR(255),
    created_at TIMESTAMP DEFAULT now(),
    PRIMARY KEY (id),
    FOREIGN KEY (user_id) REFERENCES "user"(id)
);