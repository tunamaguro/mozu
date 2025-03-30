_default:
  just --list 

alias f:= format
alias l:= lint
alias lf:= lint-fix
alias r:= ready

format:
    cargo fmt --all

format-ci:
    cargo fmt --all --check

lint:
    cargo clippy --workspace --all-targets --all-features

lint-fix:
    cargo clippy --workspace --all-targets --all-features --fix --allow-dirty --allow-staged

lint-ci:
    RUSTFLAGS="--deny warnings" cargo clippy --workspace --all-targets --all-features
    RUSTFLAGS="--deny warnings" cargo clippy --workspace --all-targets --all-features --release

test:
    cargo test --all-features --workspace

ready:
    just format
    just lint-ci
    just test