use axum::{
    Json, extract::{Path, State}, http::StatusCode
};
// use crate::models::DashboardStats;
use sqlx::Row;
use uuid::Uuid;

use crate::{
    models::{
        CreateCertificate,
        CertificateResponse,
        CertificateSummary,
        DashboardStats,
    },
    state::AppState,
};

use validator::Validate;
// Handler for the health check endpoint. It simply returns a string indicating that the API is working.
pub async fn health() -> &'static str {
    "API Working"
}

// Handler for creating a new certificate. It takes the certificate details from the request body, validates them, and inserts them into the database. It returns the ID of the newly created certificate.
pub async fn create_certificate(State(state): State<AppState>,Json(payload): Json<CreateCertificate>,) -> Result<Json<Uuid>, StatusCode> {

    let certificate_id = Uuid::new_v4();

    // Start a database transaction to ensure that the certificate and its SAN entries are inserted atomically. Validate the input payload and return a Bad Request status if validation fails. Insert the certificate details into the certificates table, and then insert each SAN entry into the san_entries table. Finally, commit the transaction and return the ID of the newly created certificate.
    let mut tx = state
        .db
        .begin()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    payload.validate()
    .map_err(|_| StatusCode::BAD_REQUEST)?;

    // Insert the certificate details into the certificates table
    sqlx::query(
        r#"
        INSERT INTO certificates
        (id, subject, issuer, expiration)
        VALUES ($1,$2,$3,$4)
        "#,
    )
    .bind(certificate_id)
    .bind(&payload.subject)
    .bind(&payload.issuer)
    .bind(payload.expiration)
    .execute(&mut *tx)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Insert each SAN entry into the san_entries table, associating it with the certificate ID
    for san in &payload.san_entries {

        sqlx::query(
            r#"
            INSERT INTO san_entries
            (certificate_id, san)
            VALUES ($1,$2)
            "#,
        )
        .bind(certificate_id)
        .bind(san)
        .execute(&mut *tx)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    // Commit the transaction to save the certificate and its SAN entries in the database, and return the ID of the newly created certificate in the response.
    tx.commit()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(certificate_id))
}

/***
 * Handler for retrieving a certificate by its ID. It takes the certificate ID from the URL path, 
 * queries the database for the certificate details and its associated SAN entries, 
 * and returns them in the response. If the certificate is not found, it returns a Not Found status.
 */
pub async fn get_certificate(State(state): State<AppState>,Path(id): Path<Uuid>) -> Result<Json<CertificateResponse>, StatusCode> {
    // Query the database for the certificate details using the provided ID. If the certificate is not found, return a Not Found status. 
    let cert = sqlx::query(
        r#"
        SELECT id, subject, issuer, expiration
        FROM certificates
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_one(&state.db)
    .await
    .map_err(|_| StatusCode::NOT_FOUND)?;

    // Query the database for the SAN entries associated with the certificate ID. Collect the SAN entries into a vector of strings. If there is an error during the query, return an Internal Server Error status.
    let san_rows = sqlx::query(
        r#"
        SELECT san
        FROM san_entries
        WHERE certificate_id = $1
        "#,
    )
    .bind(id)
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let san_entries = san_rows
        .into_iter()
        .map(|row| row.get::<String, _>("san"))
        .collect();

    // Construct the response object with the certificate details and its SAN entries, and return it in the response. 
    let response = CertificateResponse {
        id: cert.get("id"),
        subject: cert.get("subject"),
        issuer: cert.get("issuer"),
        expiration: cert.get("expiration"),
        san_entries,
    };

    Ok(Json(response))
}


/***
 * Handler for listing all certificates. It queries the database for all certificates, 
 * retrieves their details, and returns a list of certificate summaries in the response. 
 * The certificates are ordered by their expiration date in ascending order. 
 * If there is an error during the database query, it returns an Internal Server Error status.
 */
pub async fn list_certificates(State(state): State<AppState>,) -> Result<Json<Vec<CertificateSummary>>, StatusCode> {

    // Query the database for all certificates, retrieving their ID, subject, issuer, and expiration date. The results are ordered by expiration date in ascending order. If there is an error during the query, return an Internal Server Error status.
    let rows = sqlx::query(
        r#"
        SELECT id, subject, issuer, expiration
        FROM certificates
        ORDER BY expiration ASC
        "#
    )
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Map the query results to a vector of CertificateSummary structs, which include the certificate ID, subject, issuer, and expiration date. Return this list of certificate summaries in the response.
    let certificates = rows
        .into_iter()
        .map(|row| CertificateSummary {
            id: row.get("id"),
            subject: row.get("subject"),
            issuer: row.get("issuer"),
            expiration: row.get("expiration"),
        })
        .collect();

    Ok(Json(certificates))
}


/***
 * Handler for the dashboard endpoint. 
 * It queries the database to get the total number of certificates and the number of certificates that are expiring within the next 30 days. 
 * It returns these statistics in the response. 
 * If there is an error during the database queries, it returns an Internal Server Error status.
 */
pub async fn dashboard(State(state): State<AppState>,) -> Result<Json<DashboardStats>, StatusCode> {

    // Query the database to get the total number of certificates. If there is an error during the query, return an Internal Server Error status.
    let total: i64 =
        sqlx::query_scalar(
            r#"
            SELECT COUNT(*)
            FROM certificates
            "#
        )
        .fetch_one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    // Query the database to get the number of certificates that are expiring within the next 30 days. This is done by counting the certificates where the expiration date is less than or equal to the current date plus 30 days. If there is an error during the query, return an Internal Server Error status.
    let expiring: i64 =
        sqlx::query_scalar(
            r#"
            SELECT COUNT(*)
            FROM certificates
            WHERE expiration
            <= NOW() + INTERVAL '30 days'
            "#
        )
        .fetch_one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(
        DashboardStats {
            total_certificates: total,
            expiring_soon: expiring,
        }
    ))
}