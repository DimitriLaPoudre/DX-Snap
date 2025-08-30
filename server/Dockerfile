FROM rust:latest as builder

RUN apt-get update && \
    apt-get install -y postgresql postgresql-contrib && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app

COPY Cargo.toml Cargo.lock .env ./
COPY src ./src
COPY migrations ./migrations

RUN service postgresql start && \
    su - postgres -c "psql -c \"CREATE USER dx_snap_server WITH PASSWORD 'dx_snap_on_top';\"" && \
    su - postgres -c "psql -c \"CREATE DATABASE dx_snap_db OWNER dx_snap_server;\"" && \
	cargo install sqlx-cli --no-default-features --features postgres && \
    sqlx migrate run && \
	cargo build

EXPOSE 13216

CMD service postgresql start && cargo run
