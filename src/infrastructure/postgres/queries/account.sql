-- name: CreateAccount :exec
INSERT INTO accounts (id, name)
VALUES ($1, $2);

-- name: FindAccountById :one
SELECT
    id,
    name
FROM accounts
WHERE id = $1;

-- name: FindAccountByName :one
SELECT
    id,
    name
FROM accounts
WHERE name = $1;
