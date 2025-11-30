use chrono::{NaiveDate, NaiveDateTime};
use sea_orm::prelude::*;

use super::prelude::*;
use crate::domain::dto::DoctorId;
use crate::domain::dto::user::UserId;

/// Repository trait for appointment-related database operations.
pub trait AppointmentRepo {
    /// Count appointments for a doctor on a specific date (start of day to end
    /// of day).
    async fn count_doctor_appointments_on_date(
        &self,
        doctor_id: DoctorId,
        date: NaiveDate,
    ) -> AppResult<u64, DbErr>;

    /// Count appointments for a doctor in a given week (Monday to Friday).
    async fn count_doctor_appointments_in_week(
        &self,
        doctor_id: DoctorId,
        week_start: NaiveDate,
    ) -> AppResult<u64, DbErr>;

    /// Get all appointment dates for a doctor in a date range.
    async fn get_doctor_appointment_dates(
        &self,
        doctor_id: DoctorId,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> AppResult<Vec<(NaiveDateTime, String)>, DbErr>;

    /// Create a new appointment for a patient with a doctor.
    async fn create_appointment(
        &self,
        doctor_id: DoctorId,
        patient_id: UserId,
        date_time: NaiveDateTime,
    ) -> AppResult<Ulid, DbErr>;

    /// Get all appointments for a patient with doctor and specialty
    /// information.
    ///
    /// Returns tuples of (appointment, doctor_name, specialty_name).
    async fn get_patient_appointments(
        &self,
        patient_id: UserId,
    ) -> AppResult<Vec<(DbCita, String, String)>, DbErr>;
}

impl AppointmentRepo for OrmDB {
    async fn count_doctor_appointments_on_date(
        &self,
        doctor_id: DoctorId,
        date: NaiveDate,
    ) -> AppResult<u64, DbErr> {
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
    ) -> AppResult<u64, DbErr> {
        use chrono::Duration;

        use crate::adapters::repo::prelude::cita;

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
    ) -> AppResult<Vec<(NaiveDateTime, String)>, DbErr> {
        use sea_orm::QueryOrder;

        use crate::adapters::repo::prelude::cita;

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

    async fn create_appointment(
        &self,
        doctor_id: DoctorId,
        patient_id: UserId,
        date_time: NaiveDateTime,
    ) -> AppResult<Ulid, DbErr> {
        use sea_orm::ActiveValue::Set;

        use crate::adapters::repo::prelude::cita;

        let appointment_id = Ulid::new();

        let new_appointment = cita::ActiveModel {
            id: Set(appointment_id.into()),
            doctor_id: Set(Some(doctor_id.0.into())),
            paciente_id: Set(Some(patient_id.0.into())),
            asegurado_id: Set(None), // Not handling insurance for now
            fecha: Set(date_time),
            timestamp_end: Set(None), // Will be set later
            estado: Set("agendada".to_string()),
        };

        cita::Entity::insert(new_appointment).exec(self.connection()).await?;

        Ok(appointment_id)
    }

    async fn get_patient_appointments(
        &self,
        patient_id: UserId,
    ) -> AppResult<Vec<(DbCita, String, String)>, DbErr> {
        use sea_orm::{JoinType, QueryOrder};

        use crate::adapters::repo::prelude::{
            cita, doctor, doctor_especialidad, especialidad, patient,
        };

        // Query appointments with JOINs to get doctor name and specialty
        let appointments = cita::Entity::find()
            .filter(cita::Column::PacienteId.eq(patient_id.0))
            .find_also_related(doctor::Entity)
            .order_by_desc(cita::Column::Fecha)
            .all(self.connection())
            .await?;

        let mut result = Vec::new();

        for (appointment, doctor_opt) in appointments {
            let doctor_name = if let Some(doc) = doctor_opt {
                // Get specialty for this doctor
                let specialty = doctor_especialidad::Entity::find()
                    .filter(doctor_especialidad::Column::DoctorId.eq(doc.id))
                    .find_also_related(especialidad::Entity)
                    .one(self.connection())
                    .await?;

                let specialty_name = specialty
                    .and_then(|(_, esp)| esp)
                    .map(|esp| esp.nombre)
                    .unwrap_or_else(|| "Sin especialidad".to_string());

                (doc.name, specialty_name)
            } else {
                (
                    "Doctor no asignado".to_string(),
                    "Sin especialidad".to_string(),
                )
            };

            result.push((appointment, doctor_name.0, doctor_name.1));
        }

        Ok(result)
    }
}
