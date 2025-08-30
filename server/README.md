#commit keyword
| Type       | Pour...                                        |
| ---------- | ---------------------------------------------- |
| `feat`     | features related(add, change, remove)          |
| `fix`      | bug correction                                 |
| `docs`     | docs related                                   |
| `style`    | style related                                  |
| `refactor` | reorganisation not features related            |
| `test`     | tests related                                  |
| `perf`     | performance related                            |
| `ci`       | workflow related                               |
| `setup`    | build and configs related                      |


#for install the db
sudo apt update
sudo apt install postgresql postgresql-contrib
psql -c "CREATE USER dx_snap_server WITH PASSWORD 'dx_snap_on_top'; CREATE DATABASE dx_snap_db OWNER dx_snap_server;

#for setting up the db
cargo install sqlx-cli --no-default-features --features postgres
sqlx migrate run
