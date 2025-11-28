use sea_orm::prelude::*;
use super::prelude::*;
use crate::domain::dto::DoctorId;
use chrono::{NaiveDate, NaiveDateTime};

/// Repository trait for appointment-related database operations.
pub trait AppointmentRepo {
    /// Count appointments for a doctor on a specific date (start of day to end of day).
    async fn count_doctor_appointments_on_date(
        &self,
        doctor_id: DoctorId,
        date: NaiveDate,
    ) -> Result<u64, DbErr>;

    /// Count appointments for a doctor in a given week (Monday to Friday).
    async fn count_doctor_appointments_in_week(
        &self,
        doctor_id: DoctorId,
        week_start: NaiveDate,
    ) -> Result<u64, DbErr>;

    /// Get all appointment dates for a doctor in a date range.
    async fn get_doctor_appointment_dates(
        &self,
        doctor_id: DoctorId,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<Vec<(NaiveDateTime, String)>, DbErr>;
}

impl AppointmentRepo for OrmDB {
    async fn count_doctor_appointments_on_date(
        &self,
        doctor_id: DoctorId,
        date: NaiveDate,
    ) -> Result<u64, DbErr> {
        use crate::adapters::repo::prelude::cita;

        // Create datetime range for the full day
        let day_start = date.and_hms_opt(0, 0, 0).unwrap();
        let day_end = date.and_hms_opt(23, 59, 59).unwrap();

        cita::Entity::find()
            .filter(cita::Column::DoctorId.eq(doctor_id.0))
            .filter(cita::Column::Fecha.gte(day_start))
            .filter(cita::Column::Fecha.lte(day_end))
            .filter(cita::Column::Estado.ne("cancelada"))
            .count(self.connection())
            .await
    }

    async fn count_doctor_appointments_in_week(
        &self,
        doctor_id: DoctorId,
        week_start: NaiveDate,
    ) -> Result<u64, DbErr> {
        use crate::adapters::repo::prelude::cita;
        use chrono::Duration;

        // Week is Monday to Friday (5 days)
        let week_end = week_start + Duration::days(4);
        let week_start_dt = week_start.and_hms_opt(0, 0, 0).unwrap();
        let week_end_dt = week_end.and_hms_opt(23, 59, 59).unwrap();

        cita::Entity::find()
            .filter(cita::Column::DoctorId.eq(doctor_id.0))
            .filter(cita::Column::Fecha.gte(week_start_dt))
            .filter(cita::Column::Fecha.lte(week_end_dt))
            .filter(cita::Column::Estado.ne("cancelada"))
            .count(self.connection())
            .await
    }

    async fn get_doctor_appointment_dates(
        &self,
        doctor_id: DoctorId,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<Vec<(NaiveDateTime, String)>, DbErr> {
        use crate::adapters::repo::prelude::cita;
        use sea_orm::QueryOrder;

        let start_dt = start_date.and_hms_opt(0, 0, 0).unwrap();
        let end_dt = end_date.and_hms_opt(23, 59, 59).unwrap();

        let appointments = cita::Entity::find()
            .filter(cita::Column::DoctorId.eq(doctor_id.0))
            .filter(cita::Column::Fecha.gte(start_dt))
            .filter(cita::Column::Fecha.lte(end_dt))
            .filter(cita::Column::Estado.ne("cancelada"))
            .order_by_asc(cita::Column::Fecha)
            .all(self.connection())
            .await?;

        Ok(appointments
            .into_iter()
            .map(|apt| (apt.fecha, apt.estado))
            .collect())
    }
}
