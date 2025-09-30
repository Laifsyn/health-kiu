use serde::Serialize;

use super::prelude::*;
#[derive(Serialize)]
/// Handler's DTO.
pub struct ApiSpecialty {
    pub id: i16,
    pub name: String,
    pub img_path: Option<String>,
}

impl From<Specialty> for ApiSpecialty {
    fn from(especialidad: Specialty) -> Self {
        let Specialty { id, name, img_path } = especialidad;
        let id: i16 = id.0;
        let img_path = img_path.map(|p| p.to_string_lossy().to_string());
        Self { id, name, img_path }
    }
}
