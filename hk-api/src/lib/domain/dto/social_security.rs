use super::prelude::*;
use crate::domain::dto::UserId;
use crate::domain::dto::utils::id_wrapper;

id_wrapper!(
    /// ID interno del registro de seguro social
    #[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct SsId(pub Ulid)
);

pub struct InsuranceData {
    pub id: SsId,
    pub user_id: UserId,
    /// ID del usuario según cómo es identificado por la aseguradora
    pub natural_id: String,
    pub insurance_name: String,
    pub description: Option<String>,
}
