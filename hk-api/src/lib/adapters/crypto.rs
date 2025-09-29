#[derive(Default)]
pub struct PasswordHasher {
    hasher: argon2::Argon2<'static>,
}
