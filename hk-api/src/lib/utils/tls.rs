//! TLS utilities for self-generation of certificates.
//!
//! # NOTE:
//!
//! On top of requiring enabling relevant features from tls crates, building
//! also requires that relevant dependencies like CMake and NASM (for aws-lc-rs
//! feature) to be installed on the system.
use std::path::{Path, PathBuf};

use axum_server::tls_rustls::RustlsConfig;
use color_eyre::eyre::{Context, ContextCompat, Result};
use fs_err as fs;
use rustls::ServerConfig;
use rustls_pki_types::CertificateDer;
use tracing::{info, warn};

pub static PROVIDER_INIT: std::sync::LazyLock<()> =
    std::sync::LazyLock::new(|| {
        // Initialize the default crypto provider to fix rustls CryptoProvider
        // error
        if rustls::crypto::aws_lc_rs::default_provider()
            .install_default()
            .is_err()
        {
            tracing::warn!("Crypto Provider is already installed");
        }
    });

/// Initialize the TLS configuration
///
/// If the certificate and key files exist in the data directory, they are
/// loaded. Otherwise, a self-signed certificate is generated and saved to the
/// data directory.
pub fn init_certificates(data_path: impl AsRef<Path>) -> Result<ServerConfig> {
    let data_path = data_path.as_ref();

    let (cert_path, key_path) =
        pem_file_path(data_path).context("Failed to get PEM file paths")?;

    from_file(&cert_path, &key_path)
}

/// Get the certificate and key file paths in the given root path, if missing,
/// generate them.
pub fn pem_file_path(
    root_path: impl AsRef<Path>,
) -> Result<(PathBuf, PathBuf)> {
    let root_path = root_path.as_ref();
    let cert_path = root_path.join("cert.pem");
    let key_path = root_path.join("key.pem");

    if !cert_path.exists() || !key_path.exists() {
        tracing::warn!(
            r#"Certificate and key files do not exist, generating them in "{}/[REDACTED]"#,
            root_path.display()
        );
        fs::create_dir_all(root_path)
            .context("Failure to create data directory")?;

        generate(&cert_path, &key_path, None)
            .context("Failed to generate signed certificates")?;
    }

    Ok((cert_path, key_path))
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

pub async fn get_rustls_config(
    root_path: impl AsRef<Path>,
) -> Result<RustlsConfig> {
    let root_path = root_path.as_ref();
    let (cert_path, key_path) =
        pem_file_path(root_path).context("Failed to get PEM file paths")?;
    RustlsConfig::from_pem_file(cert_path.as_path(), key_path.as_path())
        .await
        .context("Failed to create RustlsConfig from PEM files")
}
