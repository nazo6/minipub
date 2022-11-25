# minipub

Mini activitypub implemention powered by SeaORM + Axum

# Prerequisites

- cargo-make (`cargo install cargo-make`)
- docker-compose
- sea-orm-cli (`cargo install sea-orm-cli`)

# Run

First, start mysql server.

```sh
cargo make start-mysql
```

Then, access `mysql://root:pwd@localhost:3308` and create "minipub" database.
Next, run migrate.

```sh
cargo make migrate
```

And now you can start server.

```sh
cargo make dev
```
