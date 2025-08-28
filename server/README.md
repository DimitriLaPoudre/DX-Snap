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
| `setup`    | build and configs related                                  |


#for install the db
sudo apt update
sudo apt install postgresql postgresql-contrib

cargo install sqlx-cli --no-default-features --features postgres
sqlx database create
sqlx migrate run
