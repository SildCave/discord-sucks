use std::path::PathBuf;

use mongodb::{
    options::{AuthMechanism, ClientOptions, Credential, Tls, TlsOptions, Compressor},
    Client,
};

pub fn prepare_mongodb_client(
    username: String,
    password: String,
    source_db: String,
) -> Result<Client, mongodb::error::Error> {
    let mut client_options = ClientOptions::default();
    let tls_options = TlsOptions::builder()
        .cert_key_file_path(PathBuf::from("configuration/mongodb/ssl/mongodb.pem"))
        .allow_invalid_certificates(true)
        .build();
    let credentials = Credential::builder()
        .mechanism(AuthMechanism::ScramSha256)
        .username(username)
        .password(password)
        .source(source_db)
        .build();

    let comporession_options = vec![
        Compressor::Snappy,
        Compressor::Zlib {
            level: Default::default(),
        },
        Compressor::Zstd {
            level: Default::default(),
        }
    ];

    client_options.tls = Some(Tls::Enabled(tls_options));
    client_options.compressors = Some(comporession_options);
    client_options.credential = Some(credentials);
    let client = Client::with_options(client_options)?;

    Ok(client)
}