use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use chrono::{Datelike, NaiveDate};
use sea_orm::prelude::Uuid;

use crate::app::services::appointments::AppointmentService;
use crate::domain::dto::doctor::DoctorId;
use crate::routes::prelude::StateApp;
use crate::repo::prelude::AppointmentRepo;

/// Query parameters for available dates endpoint
#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct AvailabilityQuery {
    /// Start date in YYYY-MM-DD format
    pub start_date: Option<String>,
    /// Number of days to look ahead (max 30)
    pub days: Option<u16>,
}

impl Default for AvailabilityQuery {
    fn default() -> Self {
        Self {
            start_date: None,
            days: Some(14), // Default to 2 weeks
        }
    }
}

/// Status of a date for appointments
#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum DateStatus {
    Available,   // Doctor has availability (< 5 appointments)
    Full,        // Doctor has 5 appointments already
    Weekend,     // Saturday or Sunday
}

/// Information about a specific date's availability
#[derive(Debug, Serialize)]
pub struct DateAvailability {
    pub date: String,           // YYYY-MM-DD format
    pub status: DateStatus,
    pub appointments_count: u64, // Current number of appointments
    pub max_appointments: u64,   // Maximum allowed (5)
}

/// Response for available dates endpoint
#[derive(Debug, Serialize)]
pub struct AvailableDatesResponse {
    pub doctor_id: String,
    pub dates: Vec<DateAvailability>,
    pub start_date: String,
    pub end_date: String,
}

/// GET /api/doctors/{doctor_id}/available-dates
///
/// Returns availability status for all dates in the range.
/// Shows available (< 5 appointments), full (5 appointments), or weekend status.
pub async fn get_available_dates(
    Path(doctor_id): Path<String>,
    Query(query): Query<AvailabilityQuery>,
    State(app): StateApp,
) -> Result<Json<AvailableDatesResponse>, (StatusCode, String)> {
    // Parse doctor ID
    let doctor_uuid = Uuid::parse_str(&doctor_id)
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid doctor ID: {}", e)))?;
    let doctor_id_domain = DoctorId(doctor_uuid);

    // Parse start date or use today
    let start_date = if let Some(date_str) = &query.start_date {
        NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
            .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid date format: {}", e)))?
    } else {
        chrono::Local::now().date_naive()
    };

    // Calculate end date (max 30 days)
    let days = query.days.unwrap_or(14).min(30) as i64;
    let end_date = start_date + chrono::Duration::days(days);

    // Build list of all dates in range with their availability status
    let mut dates = Vec::new();
    let mut current_date = start_date;

    while current_date <= end_date {
        let weekday = current_date.weekday();

        // Check if weekend
        let status = if weekday == chrono::Weekday::Sat || weekday == chrono::Weekday::Sun {
            DateStatus::Weekend
        } else {
            // Get appointment count for this date
            let count = app.db()
                .count_doctor_appointments_on_date(doctor_id_domain.clone(), current_date)
                .await
                .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to check availability".to_string()))?;

            if count >= 5 {
                DateStatus::Full
            } else {
                DateStatus::Available
            }
        };

        let appointments_count = if matches!(status, DateStatus::Weekend) {
            0
        } else {
            app.db()
                .count_doctor_appointments_on_date(doctor_id_domain.clone(), current_date)
                .await
                .unwrap_or(0)
        };

        dates.push(DateAvailability {
            date: current_date.format("%Y-%m-%d").to_string(),
            status,
            appointments_count,
            max_appointments: 5,
        });

        current_date = current_date + chrono::Duration::days(1);
    }

    Ok(Json(AvailableDatesResponse {
        doctor_id: doctor_uuid.to_string(),
        dates,
        start_date: start_date.format("%Y-%m-%d").to_string(),
        end_date: end_date.format("%Y-%m-%d").to_string(),
    }))
}

/// Request body for booking an appointment
#[derive(Debug, Deserialize)]
pub struct BookAppointmentRequest {
    pub patient_id: String,
    pub date: String, // YYYY-MM-DD format
    pub time: Option<String>, // HH:MM format (optional for now)
}

/// Response for booking an appointment
#[derive(Debug, Serialize)]
pub struct BookAppointmentResponse {
    pub appointment_id: String,
    pub doctor_id: String,
    pub patient_id: String,
    pub date_time: String,
    pub status: String,
}

/// POST /api/doctors/{doctor_id}/appointments
///
/// Books an appointment for a patient with a doctor.
/// Validates availability before creating the appointment.
pub async fn book_appointment(
    Path(_doctor_id): Path<String>,
    State(_app): StateApp,
    Json(_request): Json<BookAppointmentRequest>,
) -> Result<Json<BookAppointmentResponse>, (StatusCode, String)> {
    // TODO: Implement appointment booking
    // This will need:
    // 1. Validate doctor exists
    // 2. Validate patient exists
    // 3. Parse and validate date
    // 4. Check availability
    // 5. Create appointment in database
    // 6. Return appointment details

    Err((StatusCode::NOT_IMPLEMENTED, "Appointment booking not yet implemented".to_string()))
}
