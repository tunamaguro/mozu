-- Create accounts table
CREATE TABLE IF NOT EXISTS accounts (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL UNIQUE
);

-- Actor type Enum
CREATE TYPE actor_type AS ENUM (
    'Person',
    'Application',
    'Service',
    'Group',
    'Organization'
);

-- Create actors table
CREATE TABLE IF NOT EXISTS actors (
    id UUID PRIMARY KEY,
    -- Actor type Enum
    type ACTOR_TYPE NOT NULL,
    -- Actor name
    name VARCHAR(100) NOT NULL,
    -- Actor host
    host TEXT NOT NULL,
    inbox_url TEXT NOT NULL,
    outbox_url TEXT NOT NULL,
    shared_inbox_url TEXT,

    -- Foreign key to accounts table
    account_id UUID,
    FOREIGN KEY (account_id) REFERENCES accounts (id)
    ON UPDATE CASCADE
    ON DELETE CASCADE
);
