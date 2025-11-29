use chrono::{DateTime, Utc};

pub enum AppointmentStatus {
    Scheduled { start: DateTime<Utc>, end: Option<DateTime<Utc>> },
    Completed,
    Canceled,
    Queued { start: DateTime<Utc> },
}
