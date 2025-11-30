use axum::Json;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use chrono::{Datelike, NaiveDate};
use models::Ulid;
use sea_orm::prelude::Uuid;
use serde::{Deserialize, Serialize};

use crate::app::services::appointments::AppointmentService;
use crate::domain::dto::DoctorId;
use crate::repo::prelude::AppointmentRepo;
use crate::routes::prelude::StateApp;

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
    Available, // Doctor has availability (< 5 appointments)
    Full,      // Doctor has 5 appointments already
    Weekend,   // Saturday or Sunday
}

/// Information about a specific date's availability
#[derive(Debug, Serialize)]
pub struct DateAvailability {
    pub date: String, // YYYY-MM-DD format
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
/// Shows available (< 5 appointments), full (5 appointments), or weekend
/// status.
pub async fn get_available_dates(
    Path(doctor_id): Path<String>,
    Query(query): Query<AvailabilityQuery>,
    State(app): StateApp,
) -> Result<Json<AvailableDatesResponse>, (StatusCode, String)> {
    // Parse doctor ID
    let doctor_uuid = Uuid::parse_str(&doctor_id).map_err(|e| {
        (StatusCode::BAD_REQUEST, format!("Invalid doctor ID: {}", e))
    })?;
    let doctor_id_domain: DoctorId = doctor_uuid.into();

    // Parse start date or use today
    let start_date = if let Some(date_str) = &query.start_date {
        NaiveDate::parse_from_str(date_str, "%Y-%m-%d").map_err(|e| {
            (StatusCode::BAD_REQUEST, format!("Invalid date format: {}", e))
        })?
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
        let status = if weekday == chrono::Weekday::Sat
            || weekday == chrono::Weekday::Sun
        {
            DateStatus::Weekend
        } else {
            // Get appointment count for this date
            let count = app
                .db()
                .count_doctor_appointments_on_date(
                    doctor_id_domain.clone(),
                    current_date,
                )
                .await
                .unwrap_or(0); // Default to 0 if there's an error

            if count >= 5 { DateStatus::Full } else { DateStatus::Available }
        };

        let appointments_count = if matches!(status, DateStatus::Weekend) {
            0
        } else {
            app.db()
                .count_doctor_appointments_on_date(
                    doctor_id_domain.clone(),
                    current_date,
                )
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
    pub date: String,         // YYYY-MM-DD format
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
    Path(doctor_id): Path<String>,
    State(app): StateApp,
    Json(request): Json<BookAppointmentRequest>,
) -> Result<Json<BookAppointmentResponse>, (StatusCode, String)> {
    use crate::domain::dto::user::UserId;

    tracing::info!(
        "book_appointment handler called with doctor_id={}, request={:?}",
        doctor_id,
        request
    );

    // 1. Parse and validate doctor ID
    tracing::info!("Step 1: Parsing doctor_id: {}", doctor_id);
    let doctor_uuid = Uuid::parse_str(&doctor_id).map_err(|e| {
        tracing::error!("Failed to parse doctor_id: {}", e);
        (StatusCode::BAD_REQUEST, format!("Invalid doctor ID: {}", e))
    })?;
    tracing::info!("Step 1: Successfully parsed doctor_uuid: {}", doctor_uuid);
    let doctor_id_domain: DoctorId = doctor_uuid.into();

    // 2. Parse and validate patient ID (ULID format)
    tracing::info!("Step 2: Parsing patient_id: {}", request.patient_id);
    let patient_ulid =
        Ulid::try_from(request.patient_id.as_str()).map_err(|e| {
            tracing::error!("Failed to parse patient_id as ULID: {}", e);
            (StatusCode::BAD_REQUEST, format!("Invalid patient ID: {}", e))
        })?;
    tracing::info!(
        "Step 2: Successfully parsed patient_ulid: {}",
        patient_ulid
    );
    let patient_id_domain = UserId::from_inner(patient_ulid);

    // 3. Parse date (YYYY-MM-DD format)
    tracing::info!("Step 3: Parsing date: {}", request.date);
    let date =
        NaiveDate::parse_from_str(&request.date, "%Y-%m-%d").map_err(|e| {
            tracing::error!("Failed to parse date: {}", e);
            (
                StatusCode::BAD_REQUEST,
                format!("Invalid date format (use YYYY-MM-DD): {}", e),
            )
        })?;
    tracing::info!("Step 3: Successfully parsed date: {}", date);

    // Use provided time or default to 09:00
    let time_str = request.time.as_deref().unwrap_or("09:00");
    let time_parts: Vec<&str> = time_str.split(':').collect();
    let (hour, minute) = if time_parts.len() == 2 {
        let h = time_parts[0].parse::<u32>().unwrap_or(9);
        let m = time_parts[1].parse::<u32>().unwrap_or(0);
        (h, m)
    } else {
        (9, 0)
    };

    let date_time = date.and_hms_opt(hour, minute, 0).ok_or_else(|| {
        tracing::error!("Invalid time: {}:{}", hour, minute);
        (StatusCode::BAD_REQUEST, "Invalid time".to_string())
    })?;
    tracing::info!("Step 3: Created date_time: {}", date_time);

    // 4. Check if date is a weekend
    tracing::info!("Step 4: Checking if weekend");
    let weekday = date.weekday();
    if weekday == chrono::Weekday::Sat || weekday == chrono::Weekday::Sun {
        tracing::warn!("Rejected: weekend booking attempt");
        return Err((
            StatusCode::BAD_REQUEST,
            "Cannot book appointments on weekends".to_string(),
        ));
    }
    tracing::info!("Step 4: Not a weekend, continuing");

    // 5. Check availability (max 5 appointments per day per doctor)
    tracing::info!("Step 5: Checking availability");
    let appointments_count = app
        .db()
        .count_doctor_appointments_on_date(doctor_id_domain.clone(), date)
        .await
        .map_err(|e| {
            tracing::error!("Database error checking availability: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database error: {}", e),
            )
        })?;
    tracing::info!("Step 5: Current appointments: {}/5", appointments_count);

    if appointments_count >= 5 {
        tracing::warn!("Rejected: doctor fully booked");
        return Err((
            StatusCode::CONFLICT,
            "Doctor is fully booked for this date".to_string(),
        ));
    }

    // 6. Create appointment in database
    tracing::info!("Step 6: Creating appointment");
    let appointment_id = app
        .db()
        .create_appointment(doctor_id_domain, patient_id_domain, date_time)
        .await
        .map_err(|e| {
            tracing::error!("Failed to create appointment: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to create appointment: {}", e),
            )
        })?;
    tracing::info!("Step 6: Created appointment with ID: {}", appointment_id);

    // 7. Return appointment details
    Ok(Json(BookAppointmentResponse {
        appointment_id: appointment_id.to_string(),
        doctor_id: doctor_uuid.to_string(),
        patient_id: request.patient_id,
        date_time: date_time.format("%Y-%m-%d %H:%M").to_string(),
        status: "agendada".to_string(),
    }))
}

/// Response structure for patient appointments
#[derive(Debug, Serialize)]
pub struct PatientAppointment {
    pub id: String,
    pub doctor_name: String,
    pub specialty_name: String,
    pub date: String, // YYYY-MM-DD
    pub time: String, // HH:MM
    pub hospital: Option<String>,
    pub status: String,
}

/// GET /api/patients/{patient_id}/appointments
///
/// Returns all appointments for a specific patient with doctor and specialty
/// details.
pub async fn get_patient_appointments(
    Path(patient_id): Path<String>,
    State(app): StateApp,
) -> Result<Json<Vec<PatientAppointment>>, (StatusCode, String)> {
    use crate::domain::dto::user::UserId;

    // 1. Parse and validate patient ID (ULID format)
    let patient_ulid = Ulid::try_from(patient_id.as_str()).map_err(|e| {
        (StatusCode::BAD_REQUEST, format!("Invalid patient ID: {}", e))
    })?;
    let patient_id_domain = UserId::from_inner(patient_ulid);

    // 2. Query database for all appointments with doctor and specialty info
    let appointments =
        app.db().get_patient_appointments(patient_id_domain).await.map_err(
            |e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Database error: {}", e),
                )
            },
        )?;

    // 3. Format appointments for response
    let response: Vec<PatientAppointment> = appointments
        .into_iter()
        .map(|(apt, doctor_name, specialty_name)| {
            let date = apt.fecha.date();
            let time = apt.fecha.time();

            PatientAppointment {
                id: apt.id.to_string(),
                doctor_name,
                specialty_name,
                date: date.format("%Y-%m-%d").to_string(),
                time: time.format("%H:%M").to_string(),
                hospital: None, // Will be populated after migration
                status: apt.estado,
            }
        })
        .collect();

    Ok(Json(response))
}
