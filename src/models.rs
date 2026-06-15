use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;


#[derive(Debug, Serialize, Deserialize,Validate)]
pub struct CreateCertificate {
    #[validate(length(min = 1))]
    pub subject: String,

    #[validate(length(min = 1))]
    pub issuer: String,
    pub expiration: NaiveDateTime,
    pub san_entries: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct CertificateResponse {
    pub id: Uuid,
    pub subject: String,
    pub issuer: String,
    pub expiration: NaiveDateTime,
    pub san_entries: Vec<String>,
}

#[derive(Debug, serde::Serialize)]
pub struct CertificateSummary {
    pub id: uuid::Uuid,
    pub subject: String,
    pub issuer: String,
    pub expiration: chrono::NaiveDateTime,
}

#[derive(Debug, serde::Serialize)]
pub struct DashboardStats {
    pub total_certificates: i64,
    pub expiring_soon: i64,
}