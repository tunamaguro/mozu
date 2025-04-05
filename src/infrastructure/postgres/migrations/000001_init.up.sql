-- Create accounts table
CREATE TABLE accounts (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL UNIQUE
);
