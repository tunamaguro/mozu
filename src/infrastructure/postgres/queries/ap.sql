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
