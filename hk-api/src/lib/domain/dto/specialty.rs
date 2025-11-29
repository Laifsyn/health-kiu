use std::path::PathBuf;

use super::prelude::*;
use crate::domain::dto::utils::id_wrapper;

id_wrapper! {
    #[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct SpecialtyId(pub i16)
}

impl From<SpecialtyId> for sea_orm::Value {
    fn from(id: SpecialtyId) -> Self { sea_orm::Value::from(id.0) }
}

#[derive(Clone)]
pub struct Specialty {
    pub id: SpecialtyId,
    pub name: String,
    pub img_path: Option<PathBuf>,
}

impl Specialty {
    pub fn from_model(model: DbEspecialidad) -> Self {
        let DbEspecialidad { id, img_path, nombre: name } = model;
        Specialty {
            id: SpecialtyId(id),
            name,
            img_path: img_path.map(PathBuf::from),
        }
    }
}

impl From<DbEspecialidad> for Specialty {
    /// Delegates to [`Specialty::from_model`].
    fn from(model: DbEspecialidad) -> Self { Specialty::from_model(model) }
}
