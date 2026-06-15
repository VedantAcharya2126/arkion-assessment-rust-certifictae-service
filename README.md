# Certificate Inventory Management System

## Overview

This project implements a full-stack certificate inventory management platform using:

* Rust
* Axum
* Tokio
* PostgreSQL
* SQLx
* Next.js
* Docker
* TLS (Rustls)

The system allows users to create, retrieve, and monitor certificate metadata through a secure HTTPS-enabled API and a server-side rendered web interface.

---

# Architecture

Browser → Next.js → Rust API → PostgreSQL

The frontend uses Next.js Server Side Rendering (SSR) to retrieve certificate information from the Rust backend over HTTPS.

---

# Features

## Backend

* Create certificates
* Retrieve certificate by ID
* Retrieve all certificates
* Dashboard statistics
* PostgreSQL persistence
* SAN entry support
* Async processing
* TLS support

## Frontend

* Inventory page
* Certificate details page
* Dashboard
* SSR rendering
* HTTPS integration

---

# Technology Stack

Frontend

* Next.js
* TypeScript
* Tailwind CSS

Backend

* Rust
* Axum
* Tokio
* SQLx

Database

* PostgreSQL

Security

* Rustls
* TLS 1.3
* Self-signed certificates

---

# API Endpoints

## Health

GET /health

---

## Create Certificate

POST /addCertificates

Request:

{
"subject":"example.com",
"issuer":"Lets Encrypt",
"expiration":"2027-01-01T00:00:00",
"san_entries":[
"example.com",
"[www.example.com](http://www.example.com)"
]
}

---

## Retrieve Certificate

GET /getCertificate/{id}

---

## Retrieve All Certificates

GET /getCertificates

---

## Dashboard

GET /dashboard

Returns:

{
"total_certificates": 10,
"expiring_soon": 2
}

---

# Database Schema

certificates

* id
* subject
* issuer
* expiration

san_entries

* id
* certificate_id
* san

---

# TLS Configuration

The Rust API is secured using Rustls and a self-signed X.509 certificate.

Verification:

curl -vk https://127.0.0.1:3000/health

The frontend communicates with the backend over HTTPS.

---

# Memory Safety

Rust ownership and borrowing rules ensure:

* No dangling pointers
* No use-after-free
* No double free
* No memory leaks caused by manual allocation

---

# Concurrency

The application uses:

* Tokio Runtime
* Async Axum Handlers
* SQLx Connection Pool

allowing concurrent request processing while remaining thread-safe.

---

# Running The Project

Backend

cargo run

Frontend

npm run dev -- -p 3001

Database

docker compose up

---

# Future Improvements

* mTLS support
* JWT authentication
* SQLx migrations
* Certificate file parsing
* Certificate expiration notifications
* Kubernetes deployment

---
