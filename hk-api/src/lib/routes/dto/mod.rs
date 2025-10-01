//! Handler's DTO.
//!
//! - Converts between Requests models and BusinessLogic types.
//! - Converts between BusinessLogic types and Response models.

mod doctor;
mod id;
mod name;
mod pagination;
mod specialty;

pub use doctor::*;
pub use id::*;
pub use name::*;
pub use pagination::*;
pub use specialty::*;

mod prelude {
    #![allow(unused_imports)]
    pub use super::ApiName;
    pub use crate::domain::Paged;
    pub use crate::domain::dto::doctor::Doctor;
    pub use crate::domain::dto::specialty::Specialty;
    pub use crate::domain::dto::{DoctorId, SpecialtyId};
    pub use crate::routes::{ApiError, ErrorKind};
}
