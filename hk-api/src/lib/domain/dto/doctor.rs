use db::doctor::Model as DbDoctor;
use db::user::Model as DbUser;
use models as db;
use secrecy::SecretString;

use crate::domain::Name;
use crate::domain::dto::user::UserId;

#[derive(Clone)]
pub struct Doctor {
    pub id: UserId,
    pub cedula: String,
    pub passport: Option<String>,
    pub nombre: Name,
    pub password_hash: SecretString,
}

impl Doctor {
    pub fn from_models(doctor: DbDoctor, user: DbUser) -> Self {
        // FIXME: We should return an error instead of panicking.
        assert_eq!(
            doctor.id, user.id,
            "Doctor and User models must have the same id"
        );
        let DbDoctor { id: _, name, password_hash } = doctor;
        let DbUser { id, cedula, passport } = user;
        Doctor {
            id: UserId(id.into()),
            cedula,
            passport,
            nombre: Name::new(name),
            password_hash: SecretString::from(password_hash),
        }
    }
}
impl From<(DbDoctor, DbUser)> for Doctor {
    /// Delegates to [`Doctor::from_models`].
    fn from((doctor, user): (DbDoctor, DbUser)) -> Self {
        Self::from_models(doctor, user)
    }
}
