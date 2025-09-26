//! Handler's DTO.
//!
//! - Converts between Requests models and BusinessLogic types.
//! - Converts between BusinessLogic types and Response models.
use crate::routes::prelude;

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
