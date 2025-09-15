- [Binary Build - Required Third Party Dependencies](#binary-build---required-third-party-dependencies)
- [Running Migrations:](#running-migrations)
- [Generating Migration Files via CLI:](#generating-migration-files-via-cli)
  - [Example lib.rs](#example-librs)

# Binary Build - Required Third Party Dependencies

Building the API server requires system dependencies of [CMake](https://cmake.org/download/) and [NASM](https://www.nasm.us/) due to the usage of [rustls](https://crates.io/crates/rustls) to generate self-signed certificates

-- **FIXME**: Guard Certificate generation behind a feature gate.

# Running Migrations:

Requires [`sea-orm-cli`](https://crates.io/crates/sea-orm-cli) binary to be installed

```shell
# Install and Usage `https://crates.io/crates/sea-orm-cli` (date:2025) 
cargo install sea-orm-cli 
sea-orm-cli help

# Drop all tables from the database, then reapply all migrations:
sea-orm-cli migrate fresh -u postgresql://[username]:[password]@[hostname]:[port]/test-hk-db --verbose

# Apply pending migrations
sea-orm-cli migrate up -u postgresql://[username]:[password]@[hostname]:[port]/test-hk-db --verbose

# Rollback all applied migrations
sea-orm-cli reset up -u postgresql://[username]:[password]@[hostname]:[port]/test-hk-db --verbose

# Regenerate entities from an existing database
sea-orm-cli generate entity -u postgresql://[username]:[password]@[hostname]:[port]/test-hk-db -o models/src/entities
```

# Generating Migration Files via CLI:

Through `empiric testing`, any new code written to [`migration/src/lib.rs`] must not be written after the struct definition of `Migrator`, because otherwise will be replaced by the cli.

## Example lib.rs

```rs
use sea_orm_migration as som;
pub use sea_orm_migration::prelude::*;

mod m20250904_203638_create_tables;
mod m20250912_212231_some_name;

pub fn hello_world() {
    println!("This function is safe from being overwritten.")
}

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250904_203638_create_tables::Migration),
            Box::new(m20250912_212231_some_name::Migration),
        ]
    }
}

pub fn hello_world_erased() {
    println!("This function IS NOT safe from being overwritten.")
}

```


