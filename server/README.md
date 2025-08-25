sudo apt update
sudo apt install postgresql postgresql-contrib

## connect to the db for create an account
sudo -u postgres psql
CREATE USER dx_snap_server WITH PASSWORD 'dx_snap_on_top';
CREATE DATABASE dx_snap_db OWNER dx_snap_server;
GRANT ALL PRIVILEGES ON DATABASE dx_snap_db TO dx_snap_server;
\q

export DATABASE_URL="postgres://dx_snap_server:dx_snap_on_top@localhost/dx_snap_db"
