FROM rust:latest

COPY . /app

WORKDIR /app

RUN apt-get update && apt-get install -y --no-install-recommends postgresql postgresql-contrib && cargo install sqlx-cli --no-default-features --features postgres && /etc/init.d/postgresql start && psql -c "CREATE USER dx_snap_server WITH PASSWORD 'dx_snap_on_top'; CREATE DATABASE dx_snap_db OWNER dx_snap_server;" && sqlx migrate run && cargo build

CMD ["cargo", "run"]
