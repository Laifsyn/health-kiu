//! Layer for Object's repositories.
//!
//! This layer is responsible for interacting with the database.
//!
//! It provides an abstraction over most the databases operations.
mod doctors;
mod specialties;

/// Exports repositories, and re-exports database models.
pub mod prelude {
    pub use models::asegurado::Model as DbAsegurado;
    pub use models::cita::Model as DbCita;
    pub use models::doctor::Model as DbDoctor;
    pub use models::doctor_especialidad::Model as DbDoctorEspecialidad;
    pub use models::especialidad::Model as DbEspecialidad;
    pub use models::habitacion::Model as DbHabitacion;
    pub use models::patient::Model as DbPatient;
    pub use models::user::Model as DbUser;

    pub use super::doctors::DoctorRepo;
    pub use super::specialties::SpecialtyRepo;
}
