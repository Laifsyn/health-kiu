use dotenvy::{dotenv, from_filename};

pub fn init_env() {
    if cfg!(debug_assertions) {
        from_filename(".env.development").ok();
    }
    dotenv().ok();
}
