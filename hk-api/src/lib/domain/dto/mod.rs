//! Domain Models' [DTO][dto]. Public in [`crate::domain`].
//!
//! Converts between database models and business logic types.
//!
//! [dto]:https://en.wikipedia.org/wiki/Data_transfer_object
pub use doctor::DoctorId;
pub use patient::PatientId;
pub use specialty::SpecialtyId;
pub use user::UserId;

pub mod doctor;
pub mod patient;
pub mod specialty;
pub mod user;
/// Helper utils for Domain's DTOs.
mod utils;

mod prelude {
    #![allow(unused_imports)]
    pub use models::doctor::Model as DbDoctor;
    pub use models::especialidad::Model as DbEspecialidad;
    pub use models::patient::Model as DbPatient;
    pub use models::user::Model as DbUser;
    pub use sea_orm::prelude::*;

    pub use crate::Ulid;
}
