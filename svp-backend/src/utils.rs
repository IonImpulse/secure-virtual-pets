use crate::encryption::*;

/// Creates a self-signed certificate, and returns the certificate and key as strings
pub fn create_cert() -> Result<(String, String), Box<dyn std::error::Error>> {
    let certificate = rcgen::generate_simple_self_signed(vec!["localhost".into()])?;
    let cert = certificate.serialize_pem()?;
    let key = certificate.serialize_private_key_pem();

    Ok((cert, key))
}