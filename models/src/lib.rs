mod entities;
mod id;
pub use entities::{prelude, *};
pub use id::Id;

#[cfg(test)]
mod test {
    use sea_orm::sea_query::PostgresQueryBuilder;
    use sea_orm::{EntityTrait, QuerySelect, QueryTrait};

    use super::prelude::*;
    use crate::especialidad;
    #[test]
    fn it_works() {
        let cursor = Especialidad::find().offset(Some(5)).limit(20);
        println!("{:#}", cursor.build(sea_orm::DatabaseBackend::Postgres));
        let cursor = Especialidad::find().offset(Some(0)).limit(20);
        println!("{:#}", cursor.build(sea_orm::DatabaseBackend::Postgres));
        let cursor = Especialidad::find().offset(None).limit(20);
        println!("{:#}", cursor.build(sea_orm::DatabaseBackend::Postgres));
    }
}
