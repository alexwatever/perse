
# Perse Data
<br>

## Migrations

### Setup 

Install the [**SQLx CLI:**](https://crates.io/crates/sqlx-cli) to manage Migrations.

```bash
cargo install sqlx-cli
```

### Create and run migrations

Read more about migrations [here](https://github.com/launchbadge/sqlx/blob/main/sqlx-cli/README.md).

```bash
sqlx migrate add <name>
```

Creates a new file in `migrations/<timestamp>-<name>.sql`. Add your database schema changes to
this new file.

```bash
sqlx migrate run
```

Compares the migration history of the running database against the `migrations/` folder and runs
any scripts that are still pending.

### Reverting Migrations

If you would like to create _reversible_ migrations with corresponding "up" and "down" scripts, you use the `-r` flag when creating the first migration:

```bash
$ sqlx migrate add -r <name>
Creating migrations/20211001154420_<name>.up.sql
Creating migrations/20211001154420_<name>.down.sql
```

After that, you can run these as above:

```bash
$ sqlx migrate run
Applied migrations/20211001154420 <name> (32.517835ms)
```

And reverts work as well:

```bash
$ sqlx migrate revert
Applied 20211001154420/revert <name>
```

**Note**: All the subsequent migrations will be reversible as well.

```bash
$ sqlx migrate add <name1>
Creating migrations/20211001154420_<name>.up.sql
Creating migrations/20211001154420_<name>.down.sql
```
