use sea_orm::prelude::*;
use sea_orm::{EntityTrait, QueryFilter};

pub use super::prelude::*;
pub trait UserRepo {
    async fn fetch_user(&self, ids: &[Ulid]) -> repo::Result<Vec<DbUser>>;
}

impl UserRepo for OrmDB {
    async fn fetch_user(&self, ids: &[Ulid]) -> repo::Result<Vec<DbUser>> {
        user::Entity::find()
            .filter(user::Column::Id.is_in([1, 2]))
            .all(self.connection())
            .await
    }
}
