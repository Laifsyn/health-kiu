use super::prelude::*;
use crate::domain::dto::doctor::DoctorId;
use chrono::{Datelike, NaiveDate};

/// Maximum appointments per doctor per day
const MAX_APPOINTMENTS_PER_DAY: u64 = 5;

/// Maximum appointments per doctor per week (Monday-Friday)
const MAX_APPOINTMENTS_PER_WEEK: u64 = 25;

/// Service for appointment availability and booking logic.
pub trait AppointmentService {
    /// Check if a doctor can accept an appointment on a specific date.
    /// Returns Ok(true) if available, Ok(false) if not available.
    async fn is_doctor_available_on_date(
        &self,
        doctor_id: DoctorId,
        date: NaiveDate,
    ) -> AppResult<bool>;

    /// Get available dates for a doctor within a date range (up to 30 days).
    /// Returns list of dates where doctor has availability.
    async fn get_available_dates_for_doctor(
        &self,
        doctor_id: DoctorId,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> AppResult<Vec<NaiveDate>>;
}

impl AppointmentService for AppState {
    async fn is_doctor_available_on_date(
        &self,
        doctor_id: DoctorId,
        date: NaiveDate,
    ) -> AppResult<bool> {
        // Don't allow appointments on weekends
        let weekday = date.weekday();
        if weekday == chrono::Weekday::Sat || weekday == chrono::Weekday::Sun {
            return Ok(false);
        }

        // Check daily limit
        let daily_count = self
            .db
            .count_doctor_appointments_on_date(doctor_id.clone(), date)
            .await
            .map_err(AppError::map_err_with(
                "Failed to count daily appointments",
            ))?;

        if daily_count >= MAX_APPOINTMENTS_PER_DAY {
            return Ok(false);
        }

        // Check weekly limit (get the Monday of the week containing this date)
        let days_from_monday = date.weekday().num_days_from_monday();
        let week_start = date - chrono::Duration::days(days_from_monday as i64);

        let weekly_count = self
            .db
            .count_doctor_appointments_in_week(doctor_id, week_start)
            .await
            .map_err(AppError::map_err_with(
                "Failed to count weekly appointments",
            ))?;

        if weekly_count >= MAX_APPOINTMENTS_PER_WEEK {
            return Ok(false);
        }

        Ok(true)
    }

    async fn get_available_dates_for_doctor(
        &self,
        doctor_id: DoctorId,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> AppResult<Vec<NaiveDate>> {
        // Limit the range to 30 days max
        let max_end = start_date + chrono::Duration::days(30);
        let end_date = if end_date > max_end { max_end } else { end_date };

        let mut available_dates = Vec::new();
        let mut current_date = start_date;

        while current_date <= end_date {
            // Only check weekdays
            let weekday = current_date.weekday();
            if weekday != chrono::Weekday::Sat && weekday != chrono::Weekday::Sun {
                if self
                    .is_doctor_available_on_date(doctor_id.clone(), current_date)
                    .await?
                {
                    available_dates.push(current_date);
                }
            }
            current_date = current_date + chrono::Duration::days(1);
        }

        Ok(available_dates)
    }
}
