use sea_orm::prelude::*;
use uuid::Uuid;

use crate::{Ulid, doctor, user};
/// Composite between [`Doctor`](crate::entities::doctor::Model) and
/// [`User`](crate::entities::user::Model).
#[derive(DerivePartialModel)] // FromQueryResult is no longer needed
#[sea_orm(entity = "user::Entity", from_query_result)]
pub struct User {
    pub id: Ulid,
    pub cedula: String,
    pub passport: Option<String>,
    pub doctor: 
}

#[cfg(test)]
mod tests {
    const fn test_type<T>(_: &T, _: &T) {}
    use doctor::Model as DbDoctor;
    use user::Model as DbUser;

    use super::*;
    #[test]
    const fn models_type_matches() {
        #![allow(unreachable_code, unused_variables)]
        let DbUser { id, cedula, passport } =
            &DbUser { id: todo!(), cedula: todo!(), passport: todo!() };
        let user = &User { id: todo!(), cedula: todo!(), passport: todo!() };
        test_type(&user.cedula, cedula);

        let DbDoctor { id, name, password_hash } =
            &DbDoctor { id: todo!(), name: todo!(), password_hash: todo!() };
    }
}
