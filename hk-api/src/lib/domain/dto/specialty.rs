use std::path::PathBuf;

use super::prelude::*;
#[derive(Clone)]
#[repr(transparent)]
pub struct SpecialtyId(pub i16);

impl SpecialtyId {
    pub fn from_inner(inner: impl Into<i16>) -> Self { Self(inner.into()) }

    pub fn try_from_inner<T>(inner: T) -> Result<Self, T::Error>
    where
        T: TryInto<i16>,
    {
        Ok(Self(inner.try_into()?))
    }
}

impl From<&DbEspecialidad> for SpecialtyId {
    fn from(model: &DbEspecialidad) -> Self { Self(model.id) }
}

impl From<DbEspecialidad> for SpecialtyId {
    fn from(model: DbEspecialidad) -> Self { SpecialtyId::from(&model) }
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
    fn from(model: DbEspecialidad) -> Self { Specialty::from_model(model) }
}
