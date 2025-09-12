# Migration Rules

When editing files in this directory, follow these rules:

## Structure Requirements
- Always implement both `up` and `down` methods
- Use the custom `pk_auto` helper from `lib.rs` for auto-increment primary keys
- Include `#[derive(DeriveMigrationName)]` on Migration struct
- Include `#[derive(DeriveIden)]` on table/column enums
- Add new migrations to the `migrations()` vector in `lib.rs`

## Error Handling
- Always use `.inspect_err()` with descriptive tracing messages
- In `up`: fail fast with `?` after logging
- In `down`: collect all errors, log them, don't fail (follow existing pattern)
- Use `tracing::warn!` for error messages

## Safety & Style
- Use `.if_not_exists()` for table creation, `.if_exists()` for drops
- Test both up and down migrations
- Follow existing async/await and formatting patterns
- Run `cargo check` after creating migrations