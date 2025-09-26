use db::doctor::Model as DbDoctor;
use db::user::Model as DbUser;
use models as db;
use sea_orm::prelude::*;

use crate::domain::Name;
#[derive(Clone)]
pub struct DoctorId(Uuid);

impl From<&DbDoctor> for DoctorId {
    fn from(model: &DbDoctor) -> Self { DoctorId(model.id) }
}
impl From<DbDoctor> for DoctorId {
    fn from(model: DbDoctor) -> Self { DoctorId::from(&model) }
}

#[derive(Clone)]
pub struct Doctor {
    pub id: DoctorId,
    pub cedula: String,
    pub passport: Option<String>,
    pub nombre: Name,
}

impl Doctor {
    pub fn from_models(doctor: DbDoctor, user: DbUser) -> Self {
        let DbDoctor { id: _, name, password_hash: _ } = doctor;
        let DbUser { id, cedula, passport } = user;
        Doctor { id: DoctorId(id), cedula, passport, nombre: Name::new(name) }
    }
}
