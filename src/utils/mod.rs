pub mod dates;
pub mod hashkey;

pub use dates::{
    from_datetimestr_to_option_naivedatetime,
    from_timestamp_ms_to_naivedatetime,
    from_datetimestr_to_naivedatetime,
    pick_datetime_format,
};
pub use hashkey::HashKey;
// use openssl::ssl::{SslAcceptor, SslAcceptorBuilder, SslFiletype, SslMethod};


pub fn init_logging(crate_name: &str, logging_level: &str) {
    //! arg 1: crate_name (as define in Cargo.toml) or the binary name
    //! arg 2: logging_level:
    //! Logging levels: <crate-name>= debug | info | error
    std::env::set_var("RUST_LOG",
        &format!("actix_web=info,dt=debug,{}={}", crate_name, logging_level));
    pretty_env_logger::init();
}

// pub fn load_ssl_keys(key: &str, cert: &str) -> SslAcceptorBuilder {

//     let mut builder = SslAcceptor::mozilla_intermediate(
//         SslMethod::tls()
//     ).expect("SslAcceptor::mozilla_intermediate: failed to start");

//     builder.set_private_key_file(key, SslFiletype::PEM)
//         .expect(&format!("SSL private key: {} not found!", key));
//     builder.set_certificate_chain_file(cert)
//         .expect(&format!("SSL certificate: {} not found!", cert));

//     builder
// }
