use models::Ulid;

pub struct RegisterDbDoctor {
    pub name: String,
    pub password_hash: String,
    pub passport: Option<String>,
    pub cedula: String,
    /// Service layer might want to specify the `Ulid` for the patient.
    pub id: Option<Ulid>,
}

pub struct RegisterPatientPayload {
    pub name: String,
    pub password_hash: String,
    pub passport: Option<String>,
    pub cedula: String,
    /// Service layer might want to specify the `Ulid` for the patient.
    pub id: Option<Ulid>,
}
