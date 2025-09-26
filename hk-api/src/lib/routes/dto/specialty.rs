use models::especialidad::Model as DbSpecialty;
use serde::Serialize;

#[derive(Serialize)]
/// Handler's DTO.
pub struct Specialty {
    pub id: i16,
    pub name: String,
    pub img_path: Option<String>,
}

impl From<DbSpecialty> for Specialty {
    fn from(especialidad: DbSpecialty) -> Self {
        let DbSpecialty { id, nombre: name, img_path } = especialidad;
        Self { id, name, img_path }
    }
}
