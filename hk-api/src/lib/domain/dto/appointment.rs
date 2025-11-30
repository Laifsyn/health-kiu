use super::doctor::Doctor;
use super::patient::Patient;
use super::prelude::*;
use crate::domain::appointment_status::AppointmentStatus;
use crate::domain::dto::social_security::SsId;
use crate::domain::dto::utils::id_wrapper;

id_wrapper!(
    /// Unique identifier for an appointment.
    #[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct AppointmentId(pub Ulid)
);

pub struct Appointment {
    pub id: Ulid,
    pub doctor: Option<Doctor>,
    pub patient: Patient,
    pub ss: SsId,
    pub status: AppointmentStatus,
    pub date_time: chrono::NaiveDateTime,
    pub end_time: Option<chrono::NaiveDateTime>,
    pub hospital: Option<String>,
}
