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
    sqruff lint

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

MIGRATION_DIR := "src/infrastructure/postgres/migrations"
migrate NAME:
    migrate create -ext sql -dir {{MIGRATION_DIR}} -seq {{NAME}}

migrate_db_up:
    migrate -database ${DATABASE_URL} -path {{MIGRATION_DIR}} up

migrate_db_down:
    migrate -database ${DATABASE_URL} -path {{MIGRATION_DIR}} down

reset_db: migrate_db_down migrate_db_up

# install tools
install:
    cargo install cargo-binstall 
    cargo binstall sqruff bacon

# Start dev server
[unix]
dev:
    #!/usr/bin/bash
    if [ -z "${HOST_URL}" ]; then
        just start_serveo
        export HOST_URL=$(cat ${SERVEO_ADDR})
        SERVEO_STARTED=1
    else
        SERVEO_STARTED=0
    fi
    if [ "$SERVEO_STARTED" -eq 1 ]; then
        trap "just finish_serveo" EXIT
    fi

    bacon run-long

export SERVEO_ADDR := "serveo_addr.txt"
export SERVEO_PID := "serveo_pid.txt"
    
# Start serveo 
[unix]
start_serveo:
    #!/usr/bin/bash
    ssh -R 80:localhost:3000 serveo.net > ${SERVEO_ADDR} 2>&1 &
    echo $! > ${SERVEO_PID}

    while true; do
        if grep -q 'Forwarding HTTP traffic from' ${SERVEO_ADDR}; then
            ADDR=$(grep 'Forwarding HTTP traffic from' ${SERVEO_ADDR} | grep -o 'https://.*serveo.net')
            echo "HTTP forwarded from $ADDR"
            echo $ADDR > $SERVEO_ADDR
            break
        fi
        sleep 1
    done
    

# Kill serveo ssh and remove files
[unix]
finish_serveo:
    #!/usr/bin/bash
    if [ -f ${SERVEO_PID} ]; then
        kill $(cat ${SERVEO_PID})
        rm ${SERVEO_PID}
        echo "serveo finished"
    else
        echo "serveo is not running"
    fi
    if [ -f ${SERVEO_ADDR} ]; then
        rm ${SERVEO_ADDR}
    fi