use std::fs;
use x509_parser::prelude::*;

pub fn parse_certificate(
    path: &str,
) -> anyhow::Result<()> {

    let data = fs::read(path)?;

    let (_, cert) =
        X509Certificate::from_der(&data)?;

    println!(
        "Subject: {}",
        cert.subject()
    );

    println!(
        "Issuer: {}",
        cert.issuer()
    );

    Ok(())
}