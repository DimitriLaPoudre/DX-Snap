# Self Install

## for install the db
sudo apt update
sudo apt install postgresql postgresql-contrib
psql -c "CREATE USER dx_snap_server WITH PASSWORD 'dx_snap_on_top'; CREATE DATABASE dx_snap_db OWNER dx_snap_server;

## for setting up the db
cargo install sqlx-cli --no-default-features --features postgres
sqlx migrate run

# Docker Install

## install docker
sudo apt install docker
sudo service docker start

## create image
docker build -t dx_snap_server ./server

## run the container
docker run -p "13216:13216" dx_snap_server
