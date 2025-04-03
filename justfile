_default:
  just --list 

set dotenv-filename:=".env.dev"

alias f:= format
alias l:= lint
alias lf:= lint-fix
alias r:= ready

format:
    cargo fmt --all
    sqruff fix --force


format-ci:
    cargo fmt --all --check
    sqlruff

lint:
    cargo clippy --workspace --all-targets --all-features

lint-fix:
    cargo clippy --workspace --all-targets --all-features --fix --allow-dirty --allow-staged

lint-ci:
    RUSTFLAGS="--deny warnings" cargo clippy --workspace --all-targets --all-features
    RUSTFLAGS="--deny warnings" cargo clippy --workspace --all-targets --all-features --release

test:
    cargo test --all-features --workspace

ready: format lint-ci test

generate:
    sqlc generate -f sqlc.json

migrate NAME:
    migrate create -ext sql -dir src/infrastructure/migrations -seq {{NAME}}

migrate_db_up:
    migrate -database ${DATABASE_URL} -path src/infrastructure/migrations up

migrate_db_down:
    migrate -database ${DATABASE_URL} -path src/infrastructure/migrations down

reset_db: migrate_db_down migrate_db_up

# install tools
install:
    cargo install cargo-binstall 
    cargo binstall sqruff