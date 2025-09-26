use models::doctor::Model as DbDoctorModel;
use models::user::Model as DbUserModel;
use sea_orm::prelude::Uuid;

use super::prelude::*;
pub struct Doctor {
    /// The unique identifier of the doctor.
    pub id: Uuid,
    pub cedula: String,
    pub passport: Option<String>,
    pub name: dto::Name<'static>,
}
