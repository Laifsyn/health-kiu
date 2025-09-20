use models::especialidad;
use serde::Serialize;

#[derive(Serialize)]
pub struct Specialty {
    pub id: i16,
    pub name: String,
    pub img_path: Option<String>,
}

impl From<especialidad::Model> for Specialty {
    fn from(especialidad: especialidad::Model) -> Self {
        let especialidad::Model { id, nombre: name, img_path } = especialidad;
        Self { id, name, img_path }
    }
}
