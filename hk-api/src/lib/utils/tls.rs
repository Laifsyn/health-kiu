//! TLS utilities for self-generation of certificates.
//!
//! # NOTE:
//!
//! On top of requiring enabling relevant features from tls crates, building
//! also requires that relevant dependencies like CMake and NASM (for aws-lc-rs
//! feature) to be installed on the system.
use std::path::Path;

use actix_tls::accept::rustls_0_23::reexports::ServerConfig;
use color_eyre::eyre::{Context, ContextCompat, Result};
use fs_err as fs;
use rustls_pki_types::CertificateDer;
use tracing::{info, warn};

/// Initialize the TLS configuration
///
/// If the certificate and key files exist in the data directory, they are
/// loaded. Otherwise, a self-signed certificate is generated and saved to the
/// data directory.
pub fn init(data_path: impl AsRef<Path>) -> Result<ServerConfig> {
    // Initialize the default crypto provider to fix rustls CryptoProvider error
    rustls::crypto::aws_lc_rs::default_provider().install_default().map_err(
        |_| {
            color_eyre::eyre::eyre!("Failed to install default crypto provider")
        },
    )?;

    let data_path = data_path.as_ref();
    let cert_path = data_path.join("cert.pem");
    let key_path = data_path.join("key.pem");
    if !cert_path.exists() || !key_path.exists() {
        tracing::warn!(
            r#"Certificate and key files do not exist, generating them in "{}/{{...}}"#,
            data_path.display()
        );
        fs::create_dir_all(data_path)
            .context("unable to create data directory")?;
        generate(&cert_path, &key_path, None)
            .context("Failed to generate certificate paths")
            .context("Failed to generate signed certificates")?;
    }
    from_file(&cert_path, &key_path)
}

/// Generate a self-signed certificate and save it to the certificate and key
/// files
fn generate(
    cert_path: &Path,
    key_path: &Path,
    alternate_names: Option<Vec<String>>,
) -> Result<()> {
    debug_assert_eq!(cert_path.parent(), key_path.parent());
    warn!(
        "Generating self-signed certificates and saving to {}",
        cert_path
            .parent()
            .expect("Certificate path's parent to exists")
            .display()
    );
    // FIXME: Should we allot `localhost` as a valid alternative domain name?
    let alternate_names =
        alternate_names.unwrap_or_else(|| vec!["localhost".to_string()]);

    // FIXME: Checking the generated certificate's data, shows that it's valid
    // from 1974 to year 4095
    let certified_key = rcgen::generate_simple_self_signed(alternate_names)?;

    let cert = certified_key.cert.pem().into_bytes();
    let key = certified_key.signing_key.serialize_pem().into_bytes();
    fs::write(cert_path, &cert).context("unable to save Certificate file")?;
    fs::write(key_path, &key).context("unable to save Key file")?;
    Ok(())
}

/// Load the TLS configuration from the certificate and key files
fn from_file(cert_path: &Path, key_path: &Path) -> Result<ServerConfig> {
    debug_assert_eq!(cert_path.parent(), key_path.parent());
    info!(
        "Loading TLS configuration from `{}`",
        cert_path
            .parent()
            .expect("Certificate path's parent to exists")
            .display()
    );

    // Read certificate chain from PEM file
    let cert_pem = fs::read_to_string(cert_path)
        .context("Unable to read certificate file")?;
    let cert_chain: Vec<CertificateDer> =
        rustls_pemfile::certs(&mut cert_pem.as_bytes())
            .collect::<Result<Vec<_>, _>>()
            .context("Unable to parse certificate")?;

    // Read private key from PEM file
    let key_pem = fs::read_to_string(key_path)
        .context("Unable to read private key file")?;
    let private_key = rustls_pemfile::private_key(&mut key_pem.as_bytes())
        .context("Unable to read private key")?
        .context("No private key found")?;

    // Build ServerConfig
    ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(cert_chain, private_key)
        .context("Unable to create TLS configuration")
}
