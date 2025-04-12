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
    actor_url TEXT NOT NULL,
    inbox_url TEXT NOT NULL,
    outbox_url TEXT NOT NULL,
    shared_inbox_url TEXT,

    -- Foreign key to accounts table
    account_id UUID,
    FOREIGN KEY (account_id) REFERENCES accounts (id)
    ON UPDATE CASCADE
    ON DELETE CASCADE,

    CONSTRAINT unique_actor_name UNIQUE (name, host)
);

CREATE TABLE IF NOT EXISTS account_keys (
    account_id UUID NOT NULL,
    key_type TEXT NOT NULL,
    public_key TEXT NOT NULL,
    private_key TEXT NOT NULL,
    FOREIGN KEY (account_id) REFERENCES accounts (id)
    ON UPDATE CASCADE
    ON DELETE CASCADE,

    PRIMARY KEY (account_id, key_type)
);

-- inner table for account note
CREATE TABLE IF NOT EXISTS note_sources (
    id UUID PRIMARY KEY,
    account_id UUID NOT NULL,
    content TEXT NOT NULL,

    FOREIGN KEY (account_id) REFERENCES accounts (id)
);

-- ActivityPub Note table
CREATE TABLE IF NOT EXISTS notes (
    id UUID PRIMARY KEY,
    actor_id UUID NOT NULL,
    source_id UUID,

    content TEXT NOT NULL,
    -- ActivityPub Object id
    note_url TEXT NOT NULL,

    FOREIGN KEY (actor_id) REFERENCES actors (id)
    ON UPDATE CASCADE
    ON DELETE CASCADE,

    FOREIGN KEY (source_id) REFERENCES note_sources (id)
    ON UPDATE CASCADE
    ON DELETE CASCADE
);

-- Follow
CREATE TABLE IF NOT EXISTS follows (
    follower_id UUID NOT NULL,
    followee_id UUID NOT NULL,

    FOREIGN KEY (follower_id) REFERENCES actors (id)
    ON UPDATE CASCADE
    ON DELETE CASCADE,

    FOREIGN KEY (followee_id) REFERENCES actors (id)
    ON UPDATE CASCADE
    ON DELETE CASCADE,

    UNIQUE (follower_id, followee_id)
);