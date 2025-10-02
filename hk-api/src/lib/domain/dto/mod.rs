//! Domain Models' [DTO][dto]. Public in [`crate::domain`].
//!
//! Converts between database models and business logic types.
//!
//! [dto]:https://en.wikipedia.org/wiki/Data_transfer_object
pub use doctor::DoctorId;
pub use specialty::SpecialtyId;
pub use user::UserId;

pub mod doctor;
pub mod specialty;
pub mod user;

mod prelude {
    #![allow(unused_imports)]
    pub use models::doctor::Model as DbDoctor;
    pub use models::especialidad::Model as DbEspecialidad;
    pub use models::user::Model as DbUser;
    pub use sea_orm::prelude::*;

    pub use crate::Ulid;
}
