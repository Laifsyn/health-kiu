use db::doctor::Model as DbDoctor;
use db::user::Model as DbUser;
use models as db;
use sea_orm::prelude::*;

use crate::Ulid;
use crate::domain::Name;
#[derive(Clone)]
#[repr(transparent)]
pub struct DoctorId(pub Ulid);

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
        Doctor {
            id: DoctorId(id.into()),
            cedula,
            passport,
            nombre: Name::new(name),
        }
    }
}
