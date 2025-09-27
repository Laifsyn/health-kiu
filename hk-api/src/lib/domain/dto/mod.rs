/// Domain Models' [DTO][dto]. Public in [`crate::domain`].
///
/// Converts between database models and business logic types.
///
/// [dto]:https://en.wikipedia.org/wiki/Data_transfer_object
pub mod doctor;
pub use doctor::DoctorId;

pub mod specialty;
pub use specialty::SpecialtyId;

mod prelude {
    #![allow(unused_imports)]
    pub use db::doctor::Model as DbDoctor;
    pub use db::especialidad::Model as DbEspecialidad;
    pub use db::user::Model as DbUser;
    pub use models as db;
    pub use sea_orm::prelude::*;
}
