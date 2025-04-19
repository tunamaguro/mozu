-- name: CreateActor :exec
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
VALUES (
    $1,
    $2,
    $3,
    $4,
    $5,
    $6,
    $7,
    $8,
    $9
);


-- name: GetActor :one
SELECT
    actors.id,
    actors.type,
    actors.name,
    actors.host,
    actors.actor_url,
    actors.inbox_url,
    actors.outbox_url,
    actors.shared_inbox_url,
    actors.account_id
FROM actors
WHERE actors.id = $1;

-- name: GetActorByActorUrl :one
SELECT
    actors.id,
    actors.type,
    actors.name,
    actors.host,
    actors.actor_url,
    actors.inbox_url,
    actors.outbox_url,
    actors.shared_inbox_url,
    actors.account_id
FROM actors
WHERE actors.actor_url = $1;

-- name: GetActorByNameAndHost :one
SELECT
    actors.id,
    actors.type,
    actors.name,
    actors.host,
    actors.actor_url,
    actors.inbox_url,
    actors.outbox_url,
    actors.shared_inbox_url,
    actors.account_id
FROM actors
WHERE actors.name = $1 AND actors.host = $2;

-- name: GetActorByAccountId :one
SELECT
    actors.id,
    actors.type,
    actors.name,
    actors.host,
    actors.actor_url,
    actors.inbox_url,
    actors.outbox_url,
    actors.shared_inbox_url,
    actors.account_id
FROM actors
WHERE actors.account_id = $1;
