-- name: GetAccountActor :one
SELECT
    id,
    type,
    name,
    host,
    actor_url,
    inbox_url,
    outbox_url,
    shared_inbox_url
FROM actors
WHERE account_id = $1;

-- name: GetActorByNameAndHost :one
SELECT
    id,
    type,
    name,
    host,
    actor_url,
    inbox_url,
    outbox_url,
    shared_inbox_url
FROM actors
WHERE name = $1 AND host = $2;

-- name: UpsertActor :one
INSERT INTO actors (
    id,
    type,
    name,
    host,
    actor_url,
    inbox_url,
    outbox_url,
    shared_inbox_url,
    account_id
)
VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
ON CONFLICT (name, host) DO UPDATE
SET
type = excluded.type,
actor_url = excluded.actor_url,
inbox_url = excluded.inbox_url,
outbox_url = excluded.outbox_url,
shared_inbox_url = excluded.shared_inbox_url,
account_id = excluded.account_id
RETURNING id;

-- name: InsertAccountKey :exec
INSERT INTO account_keys (
    account_id,
    key_type,
    public_key,
    private_key
)
VALUES ($1, $2, $3, $4);

-- name: GetAccountKeys :many
SELECT
    key_type,
    public_key,
    private_key
FROM account_keys
WHERE account_id = $1;

-- name: InsertNoteSource :one
INSERT INTO note_sources (
    id,
    account_id,
    content
) VALUES ($1, $2, $3)
RETURNING id;

-- name: InsertNote :exec
INSERT INTO notes (
    id,
    actor_id,
    source_id,
    content,
    note_url
) VALUES ($1, $2, $3, $4, $5);
