mod handlers;
mod models;
mod state;

use axum::{Router, response::IntoResponse, routing::get};
mod parse_cert;
mod errors;
use axum_server::tls_rustls::RustlsConfig;
use std::net::SocketAddr;

use dotenvy::dotenv;
use sqlx::PgPool;

use state::AppState;


// The main function is the entry point of the application. It sets up the necessary configurations, connects to the database, and starts the HTTPS server.
#[tokio::main]
async fn main() {
    
    // Install the Rustls crypto provider to ensure that the necessary cryptographic algorithms are available for TLS operations.
    rustls::crypto::ring::default_provider().install_default().expect("Failed to install rustls crypto provider");
    dotenv().ok(); // Load environment variables from .env file
    
    // Get the database URL from environment variables
    let database_url =
        std::env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");
    // Connect to the PostgreSQL database using sqlx.
    let pool =
        PgPool::connect(&database_url)
            .await
            .expect("DB connection failed");
    println!("Connected to database");

    // Create the application state with the database connection pool
    let state = AppState {
        db: pool,
    };

    // Build the Axum application with the defined routes and handlers, and attach the application state to it.
    let app = Router::new()
        .route("/health", get(handlers::health))
        .route("/addCertificates", axum::routing::post(handlers::create_certificate))
        .route("/getCertificateID/{id}", axum::routing::get(handlers::get_certificate))
        .route("/certificates",get(handlers::list_certificates))
        .route("/dashboard",get(handlers::dashboard))
        .with_state(state);

    /*
    For HTTP server as per assessment 1 requirements, but since we need to serve over HTTPS, we will use axum_server with Rustls instead of axum's built-in server
     */
    // let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    // println!("Server running on port 3000");
    // axum::serve(listener, app).await.unwrap();
    // let config = RustlsConfig::from_der(parse_cert::parse_certificate("/Users/vedantacharya/Desktop/Arkion/Assignment 1/certificate-service/certs/cert.der").unwrap(), parse_cert::parse_private_key("/Users/vedantacharya/Desktop/Arkion/Assignment 1/certificate-service/certs/key.der").unwrap()).await.unwrap();
    
    // Load the TLS configuration from PEM files for the certificate and private key. This will be used to serve the application over HTTPS.
    let config = RustlsConfig::from_pem_file(
        "./certs/cert.pem",
        "./certs/key.pem",
    ).await.unwrap();

    // Define the socket address to bind the server to (localhost on port 3000) and start the HTTPS server using axum_server with the Rustls configuration.
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("HTTPS server listening on https://{}", addr);

    // Start the HTTPS server using axum_server with the Rustls configuration and serve the application.
    axum_server::bind_rustls(addr, config)
        .serve(app.into_make_service())
        .await
        .unwrap();
}


// 7bfe7d2-3be2-45fa-abed-b5f2cf62e6ae

// docker compose up --build