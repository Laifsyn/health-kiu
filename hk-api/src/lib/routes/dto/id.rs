use super::prelude::*;
/// A generic identifier that can represent different types of IDs.
pub enum Id {
    Uuid(sea_orm::prelude::Uuid),
    Integer(i64),
    Specialty(i16),
}

impl TryFrom<Id> for SpecialtyId {
    type Error = ApiError;

    fn try_from(value: Id) -> std::result::Result<Self, Self::Error> {
        let Id::Specialty(id) = value else {
            let err = ApiError {
                context: Some("Expected SpecialtyId".into()),
                source: ErrorKind::BadRequest,
            };
            return Err(err);
        };
        let id = Self::from_inner(id);
        Ok(id)
    }
}

impl TryFrom<Id> for domain_dto::UserId {
    type Error = ApiError;

    fn try_from(value: Id) -> std::result::Result<Self, Self::Error> {
        let Id::Uuid(id) = value else {
            let err = ApiError {
                context: Some("Expected DoctorId".into()),
                source: ErrorKind::BadRequest,
            };
            return Err(err);
        };

        let id = Self(id.into());
        Ok(id)
    }
}
