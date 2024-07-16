
# Perse Data
<br>

## Migrations

The below is from from SQLx, [read more about SQLx migrations here](https://github.com/launchbadge/sqlx/blob/main/sqlx-cli/README.md).  

### Setup 

Install the [**SQLx CLI:**](https://crates.io/crates/sqlx-cli) to manage Migrations.

```bash
cargo install sqlx-cli
```

### Create and run migrations

To create _reversible_ migrations with corresponding "up" and "down" scripts, you use the `-r` flag when creating the migration:

```bash
sqlx migrate add -r <name>
```

This creates the new files `migrations/<timestamp>-<name>.up.sql` and `migrations/<timestamp>-<name>.down.sql`. Add your database schema changes to these files. After that, you can run these as above:

```bash
sqlx migrate run
```

This compares the migration history of the running database against the `migrations/` folder and runs any scripts that are still pending.

### Reverting Migrations

To revert the last migration, use the following command:

```bash
sqlx migrate revert
```

**Note**: All the subsequent migrations will be reversible as well.
